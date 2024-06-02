use std::env;

mod board;
mod mcts;
mod mcts_rave;
mod minimax;
mod player;
mod rules;

fn main() {
    let args: Vec<String> = env::args().collect();
    let budget = &args[1].parse::<usize>().unwrap();
    let mut player = player::Player::new((
        player::Strategy::MCTS {
            playout_budget: *budget,
            final_solve: true,
        },
        player::Strategy::MCTSRave {
            playout_budget: *budget,
            final_solve: true,
            exploration: None,
            rave: Some(400.0)
        },
    ));
    player.play_games(100, true, 50);
}
