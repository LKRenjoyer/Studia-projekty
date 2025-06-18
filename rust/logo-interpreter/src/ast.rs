pub mod astree {
    #[derive(Clone, Debug)]
    pub enum Unop {
        Neg,
    }
    #[derive(Clone, Debug)]
    pub enum Binop {
        Add,
        Sub,
        Mul,
        Div,
        Mod,
    }
    #[derive(Clone, Debug)]
    pub enum Compop {
        Less,
        LessEq,
        More,
        MoreEq,
        Equal,
    }
    #[derive(Clone, Debug)]
    pub enum Expr {
        Function(Fun),
        UnExpr(Unop, Box<Expr>),
        BinExpr(Binop, Box<Expr>, Box<Expr>),
        Var(String),
        Number(f64),
        Word(String),
    }
    #[derive(Clone, Debug)]
    pub enum Arg {
        List(Vec<Fun>),
        Comp(Compop, Box<Expr>, Box<Expr>),
        Expr(Box<Expr>),
    }
    #[derive(Clone, Debug)]
    pub struct Fun {
        pub name: String,
        pub args: Vec<Arg>,
    }
    #[derive(Clone, Debug)]
    pub struct Declaration {
        pub name: String,
        pub args: Vec<String>,
        pub commands: Vec<Fun>,
    }
    #[derive(Clone, Debug)]
    pub struct Program {
        pub functions: Vec<Declaration>,
        pub instructions: Vec<Fun>,
    }

    pub fn cons<A>(mut v: Vec<A>, x: A) -> Vec<A> {
        v.push(x);
        v
    }
}
