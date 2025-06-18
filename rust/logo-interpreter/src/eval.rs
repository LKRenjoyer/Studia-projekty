pub mod eval {
    use crate::ast::astree::*;
    use crate::svg_editor::svg_editor::*;
    use crate::turtle::turtle::*;
    use rand::prelude::*;
    use std::collections::HashMap;

    #[derive(Clone, Debug)]
    pub enum Value {
        Number(f64),
        Word(String),
        Boolean(bool),
        Commands(Vec<Fun>),
    }
    trait AsNumber {
        fn as_number(&self) -> Result<f64, String>;
    }
    impl AsNumber for Value {
        fn as_number(&self) -> Result<f64, String> {
            if let Value::Number(n) = self {
                Ok(*n)
            } else {
                Err(format!("Nie mozna przekonwertowac {:?} na liczbe", self))
            }
        }
    }

    trait AsCommands {
        fn as_commands(&self) -> Result<Vec<Fun>, String>;
    }

    impl AsCommands for Value {
        fn as_commands(&self) -> Result<Vec<Fun>, String> {
            if let Value::Commands(cmds) = self {
                Ok(cmds.clone())
            } else {
                Err(format!(
                    "Nie mozna przekonwertowac {:?} do listy instrukcji",
                    self
                ))
            }
        }
    }

    fn eval_arg(arg: Arg, closure: &HashMap<String, Value>) -> Result<Value, String> {
        match arg {
            Arg::List(list) => Ok(Value::Commands(list)),
            Arg::Comp(comp, l_expr, r_expr) => {
                let l_res = eval_expr(*l_expr, closure)?;
                let r_res = eval_expr(*r_expr, closure)?;

                match (&l_res, &r_res) {
                    (Value::Number(i1), Value::Number(i2)) => match comp {
                        Compop::Less => Ok(Value::Boolean(i1 < i2)),
                        Compop::LessEq => Ok(Value::Boolean(i1 <= i2)),
                        Compop::More => Ok(Value::Boolean(i1 > i2)),
                        Compop::MoreEq => Ok(Value::Boolean(i1 >= i2)),
                        Compop::Equal => Ok(Value::Boolean(i1 == i2)),
                    },
                    _ => Err(format!("Blad Compop {:?} | {:?}", l_res, r_res)),
                }
            }
            Arg::Expr(expr) => eval_expr(*expr, closure),
        }
    }

    fn eval_expr(expr: Expr, closure: &HashMap<String, Value>) -> Result<Value, String> {
        match expr {
            Expr::Number(num) => Ok(Value::Number(num)),
            Expr::Word(word) => Ok(Value::Word(word)),
            Expr::UnExpr(_, body) => {
                let body = eval_expr(*body, closure)?;
                if let Value::Number(x) = body {
                    Ok(Value::Number(-x))
                } else {
                    Err(format!("Nie mozna zanegowac nie-liczby : {:?}", body))
                }
            }
            Expr::BinExpr(op, lhs, rhs) => {
                let lef = eval_expr(*lhs, closure)?;
                let rig = eval_expr(*rhs, closure)?;

                match (op, lef, rig) {
                    (Binop::Add, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                    (Binop::Sub, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a - b)),
                    (Binop::Mul, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a * b)),
                    (Binop::Div, Value::Number(a), Value::Number(b)) => {
                        if b == 0.0 {
                            Err(format!("Blad : dzielenie przez 0"))
                        } else {
                            Ok(Value::Number(a / b))
                        }
                    }
                    (Binop::Mod, Value::Number(a), Value::Number(b)) => Ok(Value::Number(a % b)),
                    (_, l, r) => Err(format!("Blad operacji na {:?} i {:?}", l, r)),
                }
            }
            Expr::Var(var) => closure
                .get(&var)
                .cloned()
                .ok_or_else(|| format!("Niezadeklarowana zmienna {}", var)),

            Expr::Function(Fun { name, args }) => {
                let evaled_args = args
                    .iter()
                    .map(|argument| eval_arg(argument.clone(), closure).unwrap())
                    .collect();

                match name.as_str() {
                    "cos" => Ok(Value::Number(nth_to_num(&evaled_args, 0)?.cos())),
                    "sin" => Ok(Value::Number(nth_to_num(&evaled_args, 0)?.sin())),
                    "exp" => Ok(Value::Number(nth_to_num(&evaled_args, 0)?.exp())),
                    "red" => Ok(Value::Word("rgb(255, 0, 0)".to_string())),
                    "green" => Ok(Value::Word("rgb(0, 255, 0)".to_string())),
                    "blue" => Ok(Value::Word("rgb(0, 0, 255)".to_string())),
                    "black" => Ok(Value::Word("rgb(0, 0, 0)".to_string())),
                    "white" => Ok(Value::Word("rgb(255, 255, 255)".to_string())),
                    "orange" => Ok(Value::Word("rgb(255,127,0)".to_string())),
                    "yellow" => Ok(Value::Word("rgb(255, 255, 0)".to_string())),
                    "violet" => Ok(Value::Word("rgb(127, 0, 255)".to_string())),
                    "random" => {
                        let val = nth_to_num(&evaled_args, 0)?;
                        let random_val = rand::thread_rng().gen_range(0..=val as u32) as f64;
                        Ok(Value::Number(random_val))
                    }
                    "pick" => {
                        let list = nth_to_commands(&evaled_args, 0)?;
                        let index = rand::thread_rng().gen_range(0..list.len());
                        eval_expr(Expr::Function(list[index].clone()), closure)
                    }
                    "repcount" => closure
                        .get("repcount")
                        .cloned()
                        .ok_or_else(|| "Repcount moze byc uzywany tylko w petli".to_string()),
                    _ => Err(format!("Niezadeklarowana funkcja {}", name)),
                }
            }
        }
    }

    fn nth_to_num(v: &Vec<Value>, n: usize) -> Result<f64, String> {
        v.get(n)
            .ok_or_else(|| format!("Brak {}-tego argumentu", n))
            .unwrap()
            .as_number()
    }

    fn nth_to_commands(v: &[Value], n: usize) -> Result<Vec<Fun>, String> {
        v.get(n)
            .ok_or_else(|| format!("Brak {}-tego argumentu", n))
            .unwrap()
            .as_commands()
    }

    fn nth_arg_to_num(
        fun: &Fun,
        closure: &HashMap<String, Value>,
        n: usize,
    ) -> Result<f64, String> {
        fun.args
            .get(n)
            .ok_or_else(|| format!("Brak {}-tego argumentu dla {:?}", n, fun))
            .and_then(|argument| eval_arg(argument.clone(), closure))
            .unwrap()
            .as_number()
    }

    fn nth_to_bool(fun: &Fun, closure: &HashMap<String, Value>, n: usize) -> Result<bool, String> {
        fun.args
            .get(n)
            .ok_or_else(|| format!("Brak {}-tego argumentu dla {:?}", n, fun))
            .and_then(|argument| eval_arg(argument.clone(), closure))
            .and_then(|val| match val {
                Value::Boolean(b) => Ok(b),
                _ => Err(format!("Agrument nie jest boolem dla {:?}", fun)),
            })
    }

    fn nth_arg_to_word(
        fun: &Fun,
        closure: &HashMap<String, Value>,
        n: usize,
    ) -> Result<String, String> {
        fun.args
            .get(n)
            .ok_or_else(|| format!("Brak {}-tego argumentu dla {:?}", n, fun))
            .and_then(|arg| eval_arg(arg.clone(), closure))
            .and_then(|val| match val {
                Value::Word(word) => Ok(word),
                _ => Err(format!("Argument {:?} nie jest slowem", val)),
            })
    }

    fn nth_arg_to_command(
        fun: &Fun,
        closure: &HashMap<String, Value>,
        n: usize,
    ) -> Result<Vec<Fun>, String> {
        fun.args
            .get(n)
            .ok_or_else(|| format!("Brak {}-tego argumentu dla {:?}", n, fun))
            .and_then(|arg| eval_arg(arg.clone(), closure))
            .and_then(|val| match val {
                Value::Commands(commands) => Ok(commands),
                _ => Err(format!("Argument nie jest komendą {:?}", val)),
            })
    }

    fn get_args(fun: &Fun, closure: &HashMap<String, Value>) -> Result<Vec<Value>, String> {
        fun.args
            .iter()
            .map(|a| eval_arg(a.clone(), closure))
            .collect()
    }

    fn exec_instruction(
        fun: Fun,
        closure: &mut HashMap<String, Value>,
        turtle: &mut Turtle,
        functions: &HashMap<String, Declaration>,
    ) -> Result<Vec<Commands>, String> {
        match fun.name.as_str() {
            "forward" | "fd" => {
                let d = nth_arg_to_num(&fun, closure, 0)?;
                let start = (turtle.x, turtle.y);
                turtle.fd(d);
                if turtle.drawing {
                    Ok(vec![Commands::Segment(
                        start.0,
                        start.1,
                        turtle.x,
                        turtle.y,
                        turtle.color.clone(),
                    )])
                } else {
                    Ok(vec![])
                }
            }
            "back" | "bk" => {
                let d = nth_arg_to_num(&fun, closure, 0)?;
                let start = (turtle.x, turtle.y);
                turtle.fd(-d);
                if turtle.drawing {
                    Ok(vec![Commands::Segment(
                        start.0,
                        start.1,
                        turtle.x,
                        turtle.y,
                        turtle.color.clone(),
                    )])
                } else {
                    Ok(vec![])
                }
            }
            "left" | "lt" => {
                let t = nth_arg_to_num(&fun, closure, 0)?;
                turtle.turn(-t);
                Ok(vec![])
            }
            "right" | "rt" => {
                let t = nth_arg_to_num(&fun, closure, 0)?;
                turtle.turn(t);
                Ok(vec![])
            }
            "pu" | "penup" => {
                turtle.drawing = false;
                Ok(vec![])
            }
            "pd" | "pendown" => {
                turtle.drawing = true;
                Ok(vec![])
            }
            "repeat" => {
                let cnt = nth_arg_to_num(&fun, closure, 0)?;
                let procedure = nth_arg_to_command(&fun, closure, 1)?;
                let mut result = Vec::new();
                let mut inner_closure = closure.clone();

                for i in 0..cnt.round() as u64 {
                    inner_closure.insert("repcount".to_string(), Value::Number(i as f64));
                    let commands =
                        exec_program(procedure.clone(), &mut inner_closure, turtle, functions)?;
                    if commands.contains(&Commands::Stop) {
                        result.extend(commands);
                        break;
                    } else {
                        result.extend(commands);
                    }
                }
                Ok(result)
            }
            "setcolor" => {
                turtle.set_color(Color::from(nth_arg_to_word(&fun, closure, 0)?));
                Ok(vec![])
            }
            "label" => {
                let text = nth_arg_to_word(&fun, closure, 0)?;
                let size = closure
                    .get("label_size")
                    .and_then(|v| match v {
                        Value::Number(s) => Some(*s as u32),
                        _ => None,
                    })
                    .unwrap_or(10);
                let (x, y, rotate, color) =
                    (turtle.x, turtle.y, turtle.angle, turtle.color.clone());
                Ok(vec![Commands::Label(text, x, y, size, rotate, color)])
            }
            "setlabelheight" => {
                let size = nth_arg_to_num(&fun, closure, 0)?;
                closure.insert("label_size".to_string(), Value::Number(size));
                Ok(vec![])
            }
            "if" => {
                if nth_to_bool(&fun, closure, 0)? {
                    let cond_instr = nth_arg_to_command(&fun, closure, 1)?;
                    exec_program(cond_instr, closure, turtle, functions)
                } else {
                    Ok(vec![])
                }
            }
            "ifelse" => {
                let block_num = match nth_to_bool(&fun, closure, 0).unwrap() {
                    true => 1,
                    false => 2,
                };
                exec_program(
                    nth_arg_to_command(&fun, closure, block_num)?,
                    closure,
                    turtle,
                    functions,
                )
            }
            "window" | "hideturtle" | "showturtle" => Ok(vec![]),
            "cls" | "clearscreen" => Ok(vec![Commands::Cls]),
            "wait" => Ok(vec![Commands::Wait(nth_arg_to_num(&fun, closure, 0)?)]),
            "stop" => Ok(vec![Commands::Stop]),
            custom_name => {
                if let Some(function) = functions.get(custom_name) {
                    if function.args.len() != fun.args.len() {
                        return Err(format!("Zla liczba parametrow funkcji : {}", custom_name));
                    }
                    let args = get_args(&fun, closure)?;
                    let mut fun_context = closure.clone();
                    for (val, arg_name) in args.into_iter().zip(&function.args) {
                        fun_context.insert(arg_name.clone(), val);
                    }
                    let fun_result = exec_program(
                        function.commands.clone(),
                        &mut fun_context,
                        turtle,
                        functions,
                    )?;
                    Ok(fun_result
                        .into_iter()
                        .take_while(|x| *x != Commands::Stop)
                        .collect())
                } else {
                    Err(format!("Function {} does not exist", custom_name))
                }
            }
        }
    }

    pub fn exec_program(
        instructions: Vec<Fun>,
        closure: &mut HashMap<String, Value>,
        turtle: &mut Turtle,
        functions: &HashMap<String, Declaration>,
    ) -> Result<Vec<Commands>, String> {
        let mut result = Vec::new();

        for instruction in instructions {
            let commands = exec_instruction(instruction, closure, turtle, functions)?;
            if commands.contains(&Commands::Stop) {
                result.extend(commands);
                break;
            } else {
                result.extend(commands);
            }
        }

        Ok(result)
    }
}
