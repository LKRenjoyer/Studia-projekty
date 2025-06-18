use crate::engine;
use engine::*;
use rand::seq::SliceRandom;
use std::cmp::{max, min, Ordering};
use std::collections::HashMap;
//use std::fs::OpenOptions;
use rand::Rng;
use std::io::{self, BufRead, Write};
use std::time::Instant;
//use std::time::Instant;

const PIECE_MULT: i64 = 100;
const RAT_IN_WATER: i64 = 25;
const RAT_ELEPHANT: i64 = 150;
const DIST_MULT: i64 = 50;

enum TTFlag {
    Exact,
    LowerBound,
    UpperBound,
}

type TTentry = (i8, TTFlag, i64);

pub struct AlphaBetaPlayer {
    curr_state: State,
    my_player: bool,
    max_depth: i8,
    zorb_tabs: (ZorbTable, ZorbTable),
    curr_hash: u64,
    transposition_table: HashMap<u64, TTentry>,
}

impl AlphaBetaPlayer {
    pub fn new(max_depth: i8) -> Self {
        let zorb_tabs: (ZorbTable, ZorbTable) = Jungle::zorb_tab();
        Self {
            curr_state: Jungle::init_state(BLACK),
            my_player: WHITE,
            max_depth,
            zorb_tabs,
            curr_hash: Jungle::init_hash(zorb_tabs),
            transposition_table: HashMap::new(),
        }
    }
    fn reset(&mut self) {
        self.curr_state = Jungle::init_state(BLACK);
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
        Self::say("RDY");
        loop {
            let (cmd, args) = Self::listen();
            match cmd.as_str() {
                "BYE" => break,
                "ONEMORE" => {
                    self.reset();
                    continue;
                }
                "UGO" => self.my_player = BLACK,
                "HEDID" => {
                    let old: Coords = (
                        args[2].parse::<i8>().unwrap(),
                        args[3].parse::<i8>().unwrap(),
                    );
                    let new: Coords = (
                        args[4].parse::<i8>().unwrap(),
                        args[5].parse::<i8>().unwrap(),
                    );
                    if old != (-1, -1) {
                        let piece: usize = {
                            let opp_pieces = if self.my_player == WHITE {
                                self.curr_state.1
                            } else {
                                self.curr_state.0
                            };
                            opp_pieces.iter().position(|&p| p == old).unwrap_or(100)
                        };
                        let delta = (new.0 - old.0, new.1 - old.1);
                        let new_state: State = Jungle::result(&self.curr_state, piece, delta);
                        self.curr_hash ^=
                            Jungle::hash_delta(&self.curr_state, piece, delta, self.zorb_tabs);
                        self.curr_state = new_state;
                    }
                }
                _ => panic!("Unexpected command {}", cmd),
            }
            let start = Instant::now();

            let (piece, delta) = self.decision(self.max_depth);
            if delta == (0, 0) {
                Self::say(&"IDO -1 -1 -1 -1");
            } else {
                let old = if self.my_player == WHITE {
                    self.curr_state.0[piece]
                } else {
                    self.curr_state.1[piece]
                };
                let new = (old.0 + delta.0, old.1 + delta.1);
                let new_state: State = Jungle::result(&self.curr_state, piece, delta);
                self.curr_hash ^=
                    Jungle::hash_delta(&self.curr_state, piece, delta, self.zorb_tabs);
                self.curr_state = new_state;

                let duration = start.elapsed().as_millis();
                eprintln!("Made move in {} ms", duration);
                Self::say(&format!("IDO {} {} {} {}", old.0, old.1, new.0, new.1));
            }
        }
    }
}

impl AlphaBetaPlayer {
    fn decision(&mut self, max_depth: i8) -> (usize, (i8, i8)) {
        let mut act = Jungle::actions(&self.curr_state);
        let mut rng = rand::rng();
        act.shuffle(&mut rng);

        let mut best_score: i64 = if self.my_player == BLACK {
            i64::MIN
        } else {
            i64::MAX
        };
        if act.is_empty() {
            (0, (0, 0))
        } else if act.len() == 1 {
            act[0]
        } else {
            let mut best_move = act[0];

            let mut alpha: i64 = i64::MIN;
            let mut beta: i64 = i64::MAX;

            for (piece, delta) in act {
                let new_state: State = Jungle::result(&self.curr_state, piece, delta);
                let new_hash: u64 = self.curr_hash
                    ^ Jungle::hash_delta(&self.curr_state, piece, delta, self.zorb_tabs);

                if Jungle::terminal(&new_state) {
                    best_move = (piece, delta);
                    break;
                }
                let score: i64 = self.alpha_beta(alpha, beta, 1, max_depth, &new_state, new_hash);

                if self.my_player == BLACK {
                    alpha = max(alpha, score);
                    if best_score < score {
                        best_score = score;
                        best_move = (piece, delta);
                    }
                } else {
                    beta = min(beta, score);
                    if best_score > score {
                        best_score = score;
                        best_move = (piece, delta);
                    }
                }
            }
            best_move
        }
    }
    fn alpha_beta(
        &mut self,
        mut alpha: i64,
        mut beta: i64,
        curr_depth: i8,
        max_depth: i8,
        state: &State,
        board_hash: u64,
    ) -> i64 {
        match Jungle::utility(&state) {
            1 => i64::MAX,
            -1 => i64::MIN,
            _ => {
                let og_alpha = alpha;
                let og_beta = beta;
                if let Some(tt_entry) = self.transposition_table.get(&board_hash) {
                    if tt_entry.0 >= (max_depth - curr_depth) {
                        match tt_entry.1 {
                            TTFlag::Exact => return tt_entry.2,
                            TTFlag::LowerBound => alpha = max(alpha, tt_entry.2),
                            TTFlag::UpperBound => beta = min(beta, tt_entry.2),
                        }
                        if alpha >= beta {
                            return tt_entry.2;
                        }
                    }
                }

                if curr_depth >= max_depth {
                    return Self::heuristic_eval(&state);
                } else {
                    let mut act = Jungle::actions(&state);
                    if curr_depth <= 3 {
                        let mut rng = rand::rng();
                        act.shuffle(&mut rng);
                    }
                    let mut best_score = if state.2 == BLACK { i64::MIN } else { i64::MAX };
                    if act.len() <= 1 {
                        let (piece, delta) = {
                            if act.is_empty() {
                                (0, (0, 0))
                            } else {
                                act[0]
                            }
                        };
                        let new_state = Jungle::result(&state, piece, delta);
                        let score = self
                            .alpha_beta(alpha, beta, curr_depth, max_depth, &new_state, board_hash);
                        score
                    } else {
                        if state.2 == BLACK {
                            for (piece, delta) in act {
                                let new_state: State = Jungle::result(&state, piece, delta);
                                let new_hash: u64 = board_hash
                                    ^ Jungle::hash_delta(&state, piece, delta, self.zorb_tabs);
                                let score = self.alpha_beta(
                                    alpha,
                                    beta,
                                    curr_depth + 1,
                                    max_depth,
                                    &new_state,
                                    new_hash,
                                );

                                best_score = max(best_score, score);
                                alpha = max(alpha, score);
                                if beta <= alpha {
                                    break;
                                }
                            }
                        } else {
                            for (piece, delta) in act {
                                let new_state: State = Jungle::result(&state, piece, delta);
                                let new_hash: u64 = board_hash
                                    ^ Jungle::hash_delta(&state, piece, delta, self.zorb_tabs);
                                let score = self.alpha_beta(
                                    alpha,
                                    beta,
                                    curr_depth + 1,
                                    max_depth,
                                    &new_state,
                                    new_hash,
                                );

                                best_score = min(best_score, score);
                                beta = min(beta, score);
                                if beta <= alpha {
                                    break;
                                }
                            }
                        }

                        let tt_flag: TTFlag = if best_score <= og_alpha {
                            TTFlag::UpperBound
                        } else if best_score >= og_beta {
                            TTFlag::LowerBound
                        } else {
                            TTFlag::Exact
                        };
                        let entry = (max_depth - curr_depth, tt_flag, best_score);
                        self.transposition_table.insert(board_hash, entry);
                        best_score
                    }
                }
            }
        }
    }

    fn heuristic_eval(state: &State) -> i64 {
        match Jungle::utility(&state) {
            1 => i64::MAX,
            -1 => i64::MIN,
            _ => {
                let mut score: i64 = 0;
                let (whites, blacks, _move_ind) = state;
                whites.iter().zip(1..=8).for_each(|(&(x, y), points)| {
                    if (x, y) != (-1, -1) {
                        let to_goal = (BLACK_BASE.0 - x).abs() + (BLACK_BASE.1 - y).abs();
                        score -= points * PIECE_MULT;
                        score -= (14 - to_goal as i64) * DIST_MULT;
                    }
                });
                blacks.iter().zip(1..=8).for_each(|(&(x, y), points)| {
                    if (x, y) != (-1, -1) {
                        let to_goal = (WHITE_BASE.0 - x).abs() + (WHITE_BASE.1 - y).abs();
                        score += points * PIECE_MULT;
                        score += (14 - to_goal as i64) * DIST_MULT * (points);
                    }
                });
                // szczur w wodzie bonus
                if WATER.contains(&whites[0]) {
                    score -= RAT_IN_WATER;
                } else if (whites[0].0 - blacks[7].0).abs() + (whites[0].1 - blacks[7].1).abs() == 1
                {
                    score -= RAT_ELEPHANT;
                }
                if WATER.contains(&blacks[0]) {
                    score += RAT_IN_WATER;
                } else if (blacks[0].0 - whites[7].0).abs() + (blacks[0].1 - whites[7].1).abs() == 1
                {
                    score += RAT_ELEPHANT;
                }
                if whites[0] != (-1, -1) {
                    score -= 200;
                }
                if blacks[0] != (-1, -1) {
                    score += 200;
                }

                score
            }
        }
    }
}
