use crate::engine;
use engine::{Coords, Jungle, State, BLACK, WHITE};
use rand::{rng, seq::IndexedRandom};
use std::collections::HashMap;
use std::io::{self, BufRead, Write};

pub struct DumberPlayer {
    my_color: bool,
    curr_state: State,
    move_pool: i32,
}

impl DumberPlayer {
    pub fn new(move_pool: i32) -> Self {
        Self {
            my_color: WHITE,
            curr_state: Jungle::init_state(BLACK),
            move_pool,
        }
    }
}
impl DumberPlayer {
    pub fn random_from_state(state: &mut State, my_color: bool) -> (i32, i32) {
        let mut rng = rand::rng();
        let mut cnt = 0;
        loop {
            if Jungle::terminal(state) {
                if state.2 == my_color {
                    break (-1, cnt);
                } else {
                    break (1, cnt);
                }
            } else {
                let acts = Jungle::actions(&state);
                if let Some(mv) = acts.choose(&mut rng) {
                    *state = Jungle::result(&state, mv.0, mv.1);
                    cnt += 1;
                } else {
                    if state.2 == my_color {
                        break (-1, cnt);
                    } else {
                        break (1, cnt);
                    }
                }
            }
        }
    }
    fn choose_move(&self) -> (usize, (i8, i8)) {
        let mut moves_to_make = self.move_pool;
        let acts = Jungle::actions(&self.curr_state);
        if acts.is_empty() {
            return (0, (0, 0));
        }
        let mut scores: HashMap<(usize, (i8, i8)), i32> = HashMap::new();
        loop {
            if moves_to_make < 0 {
                break;
            } else {
                for (piece, delta) in &acts {
                    let mut start_state: State = Jungle::result(&self.curr_state, *piece, *delta);
                    let (game_res, move_count) =
                        DumberPlayer::random_from_state(&mut start_state, self.my_color);
                    moves_to_make -= move_count;
                    if let Some(a) = scores.get(&(*piece, *delta)) {
                        scores.insert((*piece, *delta), a + game_res);
                    } else {
                        scores.insert((*piece, *delta), game_res);
                    }
                }
            }
        }
        //eprintln!("{:#?}", scores);
        *scores
            .iter()
            .max_by_key(|(&_k, &v)| v)
            .map(|(k, _v)| k)
            .unwrap()
    }
}
impl DumberPlayer {
    fn reset(&mut self) {
        self.my_color = WHITE;
        self.curr_state = Jungle::init_state(BLACK);
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
                "UGO" => self.my_color = BLACK,
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
                            let opp_pieces = if self.my_color == WHITE {
                                self.curr_state.1
                            } else {
                                self.curr_state.0
                            };
                            opp_pieces.iter().position(|&p| p == old).unwrap_or(9)
                        };
                        let delta = (new.0 - old.0, new.1 - old.1);
                        self.curr_state = Jungle::result(&self.curr_state, piece, delta);
                    } else {
                        self.curr_state = Jungle::result(&self.curr_state, 0, (0, 0));
                    }
                }
                _ => panic!("Unexpected command {}", cmd),
            }
            let (piece, delta) = self.choose_move();
            if delta == (0, 0) {
                Self::say(&"IDO -1 -1 -1 -1");
            } else {
                let old = if self.my_color == WHITE {
                    self.curr_state.0[piece]
                } else {
                    self.curr_state.1[piece]
                };
                let new = (old.0 + delta.0, old.1 + delta.1);
                self.curr_state = Jungle::result(&self.curr_state, piece, delta);
                Self::say(&format!("IDO {} {} {} {}", old.0, old.1, new.0, new.1));
            }
        }
    }
}
