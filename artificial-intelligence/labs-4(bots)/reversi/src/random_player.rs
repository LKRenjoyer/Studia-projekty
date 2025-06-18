use rand::seq::IndexedRandom;
use std::io::{self, BufRead, Write};

use crate::engine;
use engine::{Reversi, ZorbTable, BLACK, WHITE};

pub struct RandomBot {
    game: Reversi,
    my_player: usize,
    opponent: usize,
    zorb_table: ZorbTable,
}

impl RandomBot {
    pub fn new() -> Self {
        let zorb_table: ZorbTable = Reversi::set_zorbist_table();
        RandomBot {
            game: Reversi::new(zorb_table),
            my_player: WHITE, // Default, will be set by UGO if we are Black
            opponent: BLACK,
            zorb_table,
        }
    }

    fn reset_game_and_players(&mut self) {
        self.game = Reversi::new(self.zorb_table); // Re-initialize the game board
        self.my_player = WHITE; // Default to White
        self.opponent = BLACK;
        self.say("RDY");
    }

    fn say(&self, msg: &str) {
        print!("{}", msg);
        // The problem states newlines are added by the Python version's say method
        // print! already doesn't add one, println! does.
        // The Python side adds '\n', so we just print the message.
        // However, standard contest protocols usually expect the bot to send the newline.
        // Let's match the Python example which uses sys.stdout.write(msg) then sys.stdout.write('\n')
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

    fn choose_move(&mut self) -> (i8, i8) {
        // Ensure game.current_player is indeed my_player before generating actions
        // This assertion is good for debugging but might be too strict if game state sync is complex
        // assert_eq!(self.game.current_player, self.my_player, "Bot's turn, but game engine says it's opponent's turn!");

        let possible_moves = self.game.actions(self.my_player);

        let mut rng = rand::rng();
        *possible_moves
            .choose(&mut rng)
            .unwrap_or(&(possible_moves[0])) // unwrap_or is for safety if choose fails (shouldn't for non-empty)
    }

    pub fn game_loop(&mut self) {
        self.say("RDY"); // Initial ready

        loop {
            let (cmd, args) = self.listen();
            // eprintln!("# Bot received: {} {:?}", cmd, args); // Debugging line
            match cmd.as_str() {
                "BYE" => break,
                "ONEMORE" => {
                    self.reset_game_and_players();
                    continue;
                }
                "UGO" => {
                    // We play Black, opponent plays White. It's our turn.
                    self.my_player = BLACK;
                    self.opponent = WHITE;
                    // Game engine should already be set for BLACK's turn after new() or reset()
                }
                "HEDID" => {
                    // Opponent made a move. It's now our turn.
                    // args: move_timeout_str, game_timeout_str, opp_x, opp_y
                    if args.len() < 4 {
                        self.say(&format!("ERR Not enough arguments for HEDID: {:?}", args));
                        panic!("Not enough arguments for HEDID");
                    }
                    let opp_x: i8 = args[2].parse().expect("Failed to parse opponent x");
                    let opp_y: i8 = args[3].parse().expect("Failed to parse opponent y");

                    // Before opponent's move, it should be opponent's turn in game engine
                    // assert_eq!(self.game.current_player, self.opponent, "Mismatch: Expected opponent's turn before HEDID. Game player: {}, Opponent: {}", self.game.current_player, self.opponent);

                    self.game.result(opp_x, opp_y, self.opponent);

                    // After opponent's move, it should be our turn in game engine
                    assert_eq!(
                        self.game.current_player, self.my_player,
                        "Mismatch: Expected my turn after HEDID. Game player: {}, My player: {}",
                        self.game.current_player, self.my_player
                    );
                }
                _ => {
                    let err_msg = format!(
                        "ERR Unimplemented or unexpected command: {} {:?}",
                        cmd, args
                    );
                    self.say(&err_msg);
                    // For a robust bot, you might not want to panic, but for a simple one it's okay.
                    panic!("{}", err_msg);
                }
            }

            // It's our turn to make a move (either after UGO or HEDID)
            let (my_x, my_y) = self.choose_move();

            // Before our move, it should be our turn in game engine
            assert_eq!(
                self.game.current_player, self.my_player,
                "Mismatch: Expected my turn before my IDO. Game player: {}, My player: {}",
                self.game.current_player, self.my_player
            );

            self.game.result(my_x, my_y, self.my_player);

            // After our move, it should be opponent's turn in game engine
            assert_eq!(
                self.game.current_player, self.opponent,
                "Mismatch: Expected opponent's turn after my IDO. Game player: {}, Opponent: {}",
                self.game.current_player, self.opponent
            );

            self.say(&format!("IDO {} {}", my_x, my_y));
        }
    }
}
