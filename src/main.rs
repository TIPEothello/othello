use std::env;

mod board;
mod mcts;
mod mcts_rave;
mod minimax;
mod player;
mod rules;

fn main() {
    let args: Vec<String> = env::args().collect();
    let budget1 = &args[1].parse::<u8>().unwrap();
    let budget2 = &args[2].parse::<usize>().unwrap();
    let mut player = player::Player::new((
        player::Strategy::Minimax { depth: *budget1 } ,
        player::Strategy::MCTSRave {
            playout_budget: *budget2,
            final_solve: true,
        },
    ));
    player.play_games(100, true);
}
