use std::env;

mod board;
mod mcts;
mod mcts_rave;
mod minimax;
mod player;
mod rules;

fn main() {
    let args: Vec<String> = env::args().collect();
    let budget2 = args[1].parse::<usize>().unwrap();
    let mut player = player::Player::new((
        player::Strategy::MCTSRave {
            playout_budget: budget2,
            final_solve: true,
            exploration: Some(0.7),
            rave: Some(750.0)
        },
        player::Strategy::Manual
    ));
    player.progressive_play();
}
