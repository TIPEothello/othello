/*
 File: main.rs
 Created Date: 21 Mar 2023
 Author: realbacon
 -----
 Last Modified: 5/09/2023 03:12:11
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

mod board;
mod mcts;
mod minimax;
mod player;
mod rules;

#[tokio::main(worker_threads = 3, flavor = "multi_thread")]
async fn main() {
    let mut player = player::Player::new((
        player::Strategy::MCTS { playout_budget: 15000 },
        player::Strategy::Minimax { depth: 4 },
    ));
    player.progressive_play();
}
