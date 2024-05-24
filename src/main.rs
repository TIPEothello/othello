use std::env;

mod board;
mod mcts;
mod minimax;
mod player;
mod rules;

fn main() {
    let args: Vec<String> = env::args().collect();
    let budget1 = &args[1].parse::<usize>().unwrap();
    let budget2 = &args[2].parse::<usize>().unwrap();
    let mut player = player::Player::new((
        player::Strategy::MCTS {
            playout_budget: *budget1,
            final_solve: true,
        },
        player::Strategy::MCTS {
            playout_budget: *budget2,
            final_solve: false,
        },
    ));
    player.play_games(100, true);
}
