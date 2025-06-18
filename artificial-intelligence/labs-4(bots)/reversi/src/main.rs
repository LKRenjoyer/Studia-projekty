mod alpha_beta;
mod engine;
mod light_engine;
mod mcts;
mod random_player;
use alpha_beta::AlphaBetaPlayer;
use mcts::MCTSBot;
use random_player::RandomBot;

fn main() {
    let mut player = AlphaBetaPlayer::new(500);
    player.game_loop();
}
