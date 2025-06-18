use crate::light_engine;
use light_engine::{Move, Reversi, State, BLACK, WHITE};
use rand::Rng;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::rc::Rc;
use std::time::Instant;

static EXP_CONST: f32 = 1.41421356237;

struct Node {
    pub state: State,
    pub visits: u32,
    pub wins: f32,
    pub children: HashMap<Move, Rc<RefCell<Node>>>,
    pub untried_moves: Vec<Move>,
}
impl Node {
    fn evaluate(&self, parent_visits: u32) -> f32 {
        if self.visits == 0 {
            f32::INFINITY
        } else {
            let exploitation = self.wins / self.visits as f32;
            let exploration = EXP_CONST * ((parent_visits as f32).ln() / self.visits as f32).sqrt();
            exploitation + exploration
        }
    }
    pub fn new(state: State) -> Self {
        Node {
            state,
            visits: 0,
            wins: 0.0,
            children: HashMap::new(),
            untried_moves: Reversi::actions(&state),
        }
    }
}

pub struct MonteCarloTree {
    root: Rc<RefCell<Node>>,
    my_player: bool,
}

impl MonteCarloTree {
    pub fn new(root_state: State, color: bool) -> Self {
        let root_node_data = Node::new(root_state);
        Self {
            root: Rc::new(RefCell::new(root_node_data)),
            my_player: color,
        }
    }

    pub fn choose_move(&mut self, millis_for_move: i32) -> (Move, i32) {
        let start = Instant::now();
        let mut cnt = 0;
        loop {
            self.run_simulation();
            cnt += 1;
            if start.elapsed().as_millis() as i32 >= millis_for_move {
                break;
            }
        }

        let root_node_borrow = self.root.borrow();

        (
            root_node_borrow
                .children
                .iter()
                .max_by_key(|&(_, child_node_rc)| child_node_rc.borrow().visits)
                .map(|(&mv, _)| mv)
                .unwrap_or((-1, -1)),
            cnt,
        )
    }

    fn run_simulation(&mut self) {
        let mut game_path: Vec<Rc<RefCell<Node>>> = Vec::with_capacity(100);

        let leaf = {
            let mut current_node = Rc::clone(&self.root);

            loop {
                game_path.push(Rc::clone(&current_node));

                if Reversi::terminal(&current_node.borrow().state) {
                    break None;
                }

                if !current_node.borrow().untried_moves.is_empty() {
                    let new_child = {
                        let mut parent = current_node.borrow_mut();
                        let mv = parent.untried_moves.pop().unwrap();
                        let new_state = Reversi::result(&parent.state, mv);

                        let child = Rc::new(RefCell::new(Node::new(new_state)));
                        parent.children.insert(mv, Rc::clone(&child));
                        child
                    };
                    game_path.push(Rc::clone(&new_child));
                    break Some(new_child);
                } else {
                    let best = {
                        let node_ref = current_node.borrow();
                        let parent_visits = node_ref.visits;
                        node_ref
                            .children
                            .values()
                            .max_by(|a, b| {
                                let ua = a.borrow().evaluate(parent_visits);
                                let ub = b.borrow().evaluate(parent_visits);
                                ua.partial_cmp(&ub).unwrap()
                            })
                            .unwrap()
                            .clone()
                    };
                    current_node = best;
                }
            }
        };

        let sim_reward = if let Some(leaf_rc) = &leaf {
            let mut state: State = leaf_rc.borrow_mut().state;

            while !Reversi::terminal(&state) {
                let moves = Reversi::actions(&state);
                let rand_move: Move = moves[rand::rng().random_range(0..moves.len())];
                state = Reversi::result(&state, rand_move);
            }

            match Reversi::utility(&(state.0, self.root.borrow().state.1)).signum() {
                0 => 0.5,
                1 if self.my_player == BLACK => 1.0,
                -1 if self.my_player == WHITE => 1.0,
                _ => 0.0,
            }
        } else {
            match Reversi::utility(&self.root.borrow().state).signum() {
                0 => 0.5,
                1 if self.my_player == BLACK => 1.0,
                -1 if self.my_player == WHITE => 1.0,
                _ => 0.0,
            }
        };

        // 3) BACKPROPAGATION
        for node_rc in game_path {
            let mut node = node_rc.borrow_mut();
            node.visits += 1;
            let win = if node.state.1 == self.my_player {
                sim_reward
            } else {
                1.0 - sim_reward
            };
            node.wins += win;
        }
    }

    pub fn move_root(&mut self, mv: Move) {
        let curr_state = self.root.borrow().state;
        let new_root = self
            .root
            .borrow_mut()
            .children
            .remove(&mv)
            .unwrap_or_else(|| {
                let new_state = Reversi::result(&curr_state, mv);
                Rc::new(RefCell::new(Node::new(new_state)))
            });
        self.root = new_root;
    }
}

pub struct MCTSBot {
    state: State,
    my_player: bool,
    millis_per_mv: i32,
    tree: MonteCarloTree,
}

impl MCTSBot {
    pub fn new(millis_per_mv: i32) -> Self {
        MCTSBot {
            state: Reversi::init_state(),
            my_player: WHITE,
            millis_per_mv,
            tree: MonteCarloTree::new(Reversi::init_state(), WHITE),
        }
    }

    fn reset_game_and_players(&mut self) {
        self.state = Reversi::init_state();
        self.my_player = WHITE;
        self.tree = MonteCarloTree::new(Reversi::init_state(), WHITE);
        self.say("RDY");
    }

    fn say(&self, msg: &str) {
        print!("{}", msg);
        println!(); // Explicitly add newline
        io::stdout().flush().unwrap();
    }

    fn listen(&self) -> (String, Vec<String>) {
        let mut line = String::new();
        match io::stdin().lock().read_line(&mut line) {
            Ok(0) => ("BYE".to_string(), vec![]), // EOF treated as BYE
            Ok(_) => {
                let parts: Vec<String> = line.trim().split_whitespace().map(String::from).collect();
                if parts.is_empty() {
                    ("BYE".to_string(), vec![]) // Empty line treated as BYE
                } else {
                    (parts[0].clone(), parts.into_iter().skip(1).collect())
                }
            }
            Err(_) => ("BYE".to_string(), vec![]), // Error treated as BYE
        }
    }

    pub fn game_loop(&mut self) {
        self.say("RDY");
        loop {
            let (cmd, args) = self.listen();

            match cmd.as_str() {
                "BYE" => break,
                "ONEMORE" => {
                    self.reset_game_and_players();
                    continue;
                }
                "UGO" => {
                    self.tree.my_player = BLACK;
                    self.tree = MonteCarloTree::new(Reversi::init_state(), BLACK);
                }
                "HEDID" => {
                    if args.len() < 4 {
                        let err_msg = format!("ERR Not enough arguments for HEDID: {:?}", args);
                        self.say(&err_msg);
                        panic!("{}", err_msg);
                    }
                    let opp_x: i8 = args[2].parse().expect("Failed to parse opponent x");
                    let opp_y: i8 = args[3].parse().expect("Failed to parse opponent y");
                    let mv: Move = (opp_x, opp_y);
                    self.state = Reversi::result(&self.state, mv);
                    self.tree.move_root(mv);
                }
                _ => {
                    let err_msg = format!(
                        "ERR Unimplemented or unexpected command: {} {:?}",
                        cmd, args
                    );
                    self.say(&err_msg);
                    panic!("{}", err_msg);
                }
            }
            //eprintln!("NOW IN STATE {:?}", self.tree.root.borrow().state);
            let start = Instant::now();
            let (mv, cnt) = self.tree.choose_move(self.millis_per_mv);
            let elapsed = start.elapsed().as_millis();
            eprintln!("{} calculations took : {}", cnt, elapsed);

            self.state = Reversi::result(&self.state, mv);
            self.tree.move_root(mv);
            self.say(&format!("IDO {} {}", mv.0, mv.1));
            //eprintln!("NOW IN STATE {:?}", self.tree.root.borrow().state);
        }
    }
}
