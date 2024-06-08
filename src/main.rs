use std::env;

mod board;
mod mcts;
mod minimax;
mod player;
mod rules;

fn main() {
    let _rayon_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .unwrap();
    let args: Vec<String> = env::args().collect();
    let budget1 = &args[1].parse::<usize>().unwrap();
    let budget2 = &args[2].parse::<u8>().unwrap();
    let games = &args
        .get(3)
        .unwrap_or(&String::from("100"))
        .parse::<u32>()
        .unwrap_or(100);
    let mut player = player::Player::new((
        player::Strategy::Minimax { depth: *budget2 },
        player::Strategy::MCTS {
            playout_budget: *budget1,
            final_solve: true,
        },
    ));
    println!("exploration constant: {}", mcts::EXPLORATION_PARAMETER);
    player.play_games(*games, true, 50);
}
