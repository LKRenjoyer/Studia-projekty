use crate::engine;
use engine::{Reversi, ZorbTable, BLACK, FIELD_SCORE, WHITE};
use std::cmp::{max, min, Ordering};
use std::collections::HashMap;
//use std::fs::OpenOptions;
use std::io::{self, BufRead, Write};
use std::time::Instant;
//use std::time::Instant;

type TTentry = (i8, i8, i32);
const INF: i32 = std::i32::MAX;
const LOWER_BOUND: i8 = 0;
const EXACT: i8 = 1;
const UPPER_BOUND: i8 = 2;

pub struct AlphaBetaPlayer {
    game: Reversi,
    my_player: usize,
    transposition_table: HashMap<u64, TTentry>,
    zorb_table: ZorbTable,
    millis_per_move: u32,
}

impl AlphaBetaPlayer {
    pub fn new(millis_per_move: u32) -> Self {
        let zorb_tab: ZorbTable = Reversi::set_zorbist_table();
        Self::say("RDY");
        AlphaBetaPlayer {
            game: Reversi::new(zorb_tab),
            my_player: WHITE,
            transposition_table: HashMap::new(),
            zorb_table: zorb_tab,
            millis_per_move,
        }
    }
    fn reset(&mut self) {
        self.game = Reversi::new(self.zorb_table);
        self.my_player = WHITE;
        self.transposition_table = HashMap::new();
        Self::say("RDY");
    }
    fn say(msg: &str) {
        println!("{}", msg);
        io::stdout().flush().unwrap();
    }
    fn listen() -> (String, Vec<String>) {
        let mut line = String::new();
        match io::stdin().lock().read_line(&mut line) {
            Ok(0) => ("BYE".to_string(), vec![]),
            Ok(_) => {
                let parts: Vec<String> = line.trim().split_whitespace().map(String::from).collect();
                if parts.is_empty() {
                    ("BYE".to_string(), vec![])
                } else {
                    (parts[0].clone(), parts[1..].to_vec())
                }
            }
            Err(_) => ("BYE".to_string(), vec![]),
        }
    }
    pub fn game_loop(&mut self) {
        /*
        let log_file = "ab.log";
        if let Ok(mut file) = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(log_file)
        {
            writeln!(file, "Round,Duration_ms,FreeFields,Depth").ok();
        }
        let mut round = 0;
        */
        loop {
            //println!("{}", self.game.to_str());
            let (cmd, args) = Self::listen();
            match cmd.as_str() {
                "BYE" => break,
                "ONEMORE" => {
                    self.reset();
                    continue;
                }
                "UGO" => self.my_player = BLACK,
                "HEDID" => {
                    let opp_x: i8 = args[2].parse::<i8>().unwrap();
                    let opp_y: i8 = args[3].parse::<i8>().unwrap();
                    self.game.result(opp_x, opp_y, self.my_player ^ 3);
                }
                _ => panic!("Unexpected command {}", cmd),
            }
            let start = Instant::now();
            let mut duration: u128 = 0u128;
            let free = self.game.free_fields;
            let mut depth = Self::get_depth(free);
            let (mut my_x, mut my_y) = (-1, -1);
            loop {
                (my_x, my_y) = self.decision(depth);
                duration = start.elapsed().as_millis();
                if duration < ((self.millis_per_move / 5) as u128) && depth < free {
                    depth += 1;
                } else {
                    break;
                }
            }
            eprintln!(
                "Move took {} ms {} deep {} free fields",
                duration, depth, free
            );
            self.game.result(my_x, my_y, self.my_player);
            /*
            round += 1;

            if let Ok(mut file) = OpenOptions::new().append(true).open(log_file) {
                writeln!(file, "{},{},{},{}", round, duration, free, depth).ok();
            }
            */
            Self::say(&format!("IDO {} {}", my_x, my_y));
        }
    }
}

// Section for alpha_beta : select_move, alpha_beta, order_moves, heuristic_eval
impl AlphaBetaPlayer {
    fn decision(&mut self, max_depth: i8) -> (i8, i8) {
        let mut act = self.game.actions(self.my_player);

        let mut best_score = if self.my_player == BLACK { -INF } else { INF };
        let mut best_move: (i8, i8) = act[0];

        if act.len() == 1 {
            best_move
        } else {
            let mut alpha = -INF;
            let mut beta = INF;
            act.sort_by(|a, b| self.eval_move(a, b));

            for (x, y) in act {
                self.game.result(x, y, self.my_player);
                let score = self.alpha_beta(alpha, beta, 1, max_depth, self.my_player ^ 3);
                self.game.undo();

                if self.my_player == BLACK {
                    alpha = max(alpha, score);
                    if best_score < score {
                        best_score = score;
                        best_move = (x, y);
                    }
                } else {
                    beta = min(beta, score);
                    if best_score > score {
                        best_score = score;
                        best_move = (x, y);
                    }
                }
            }

            best_move
        }
    }

    fn alpha_beta(
        &mut self,
        mut alpha: i32,
        mut beta: i32,
        curr_depth: i8,
        max_depth: i8,
        player: usize,
    ) -> i32 {
        if self.game.terminal() {
            let util = self.game.utility();
            if util < 0 {
                return -INF;
            } else if util == 0 {
                return 0;
            } else {
                return INF;
            }
        } else if curr_depth >= max_depth {
            return Reversi::heuristic_eval(self.game.current_board);
        } else {
            let board_hash = self.game.current_hash;

            if let Some(&(depth, flag, score)) = self.transposition_table.get(&board_hash) {
                if depth >= max_depth - curr_depth {
                    match flag {
                        LOWER_BOUND => alpha = max(score, alpha),
                        EXACT => return score,
                        UPPER_BOUND => beta = min(score, beta),
                        _ => unreachable!(),
                    }
                    if alpha >= beta {
                        return score;
                    }
                }
            }
            let mut act = self.game.actions(player);
            let best_move = act[0];
            let mut best_score = if player == BLACK { -INF } else { INF };
            if act.len() == 1 {
                self.game.result(best_move.0, best_move.1, player);
                let score = self.alpha_beta(alpha, beta, curr_depth, max_depth, player ^ 3);
                self.game.undo();
                score
            } else {
                if curr_depth <= 2 {
                    act.sort_by(|a, b| self.eval_move(a, b));
                }
                if player == BLACK {
                    for (x, y) in act {
                        self.game.result(x, y, player);
                        let score =
                            self.alpha_beta(alpha, beta, curr_depth + 1, max_depth, player ^ 3);
                        self.game.undo();

                        best_score = max(best_score, score);
                        alpha = max(alpha, score);
                        if beta <= alpha {
                            break;
                        }
                    }
                } else {
                    for (x, y) in act {
                        self.game.result(x, y, player);
                        let score =
                            self.alpha_beta(alpha, beta, curr_depth + 1, max_depth, player ^ 3);
                        self.game.undo();

                        best_score = min(best_score, score);
                        beta = min(beta, score);
                        if beta <= alpha {
                            break;
                        }
                    }
                }
                let flag = if best_score <= alpha {
                    UPPER_BOUND
                } else if best_score >= beta {
                    LOWER_BOUND
                } else {
                    EXACT
                };
                self.transposition_table
                    .insert(board_hash, (max_depth - curr_depth, flag, best_score));
                best_score
            }
        }
    }

    fn get_depth(free_fields: i8) -> i8 {
        match free_fields {
            52..=60 => 8,
            27..52 => 6,
            17..27 => 8,
            14..17 => 10,
            _ => free_fields,
        }
    }

    fn eval_move(&mut self, a: &(i8, i8), b: &(i8, i8)) -> Ordering {
        let a_score: i32 = FIELD_SCORE[a.0 as usize][a.1 as usize].into();
        let b_score: i32 = FIELD_SCORE[b.0 as usize][b.1 as usize].into();
        b_score.partial_cmp(&a_score).unwrap_or(Ordering::Equal)
    }
}
