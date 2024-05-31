use std::env;

mod board;
mod mcts;
mod minimax;
mod player;
mod rules;

fn main() {
    let args: Vec<String> = env::args().collect();
    let budget1 = &args[1].parse::<usize>().unwrap();
    let budget2 = &args[2].parse::<u8>().unwrap();
    let mut player = player::Player::new((
        player::Strategy::MCTS {
            playout_budget: *budget1,
            final_solve: true,
        },
        player::Strategy::Minimax { depth: *budget2 }

,
    ));
    player.play_games(100, true, 70);
}
