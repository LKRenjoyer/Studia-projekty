mod abplayer;
//mod dumber;
mod engine;
use abplayer::AlphaBetaPlayer;
//use dumber::DumberPlayer;
// use engine::{Jungle, BLACK};

fn main() {
    let mut player = AlphaBetaPlayer::new(5);
    player.game_loop();

    //  let mut player = DumberPlayer::new(20000);
    //player.game_loop();
}
