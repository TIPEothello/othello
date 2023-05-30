/*
 File: main.rs
 Created Date: 21 Mar 2023
 Author: realbacon
 -----
 Last Modified: 30/05/2023 01:45:50
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

#[tokio::main(worker_threads = 100)]
async fn main() {
    let mut player = player::Player::new(Some((
        player::Strategy::Manual,
        player::Strategy::MinimaxTree { depth: 6 },
    )));
    //player.progressive_play();

    let result = player.progressive_play();
    /*println!(
        "Win ratio : White {}% ({}) - Black {}% ({})",
        result.0 as f32 / (result.0 + result.1 + result.2) as f32 * 100.0,
        result.0,
        result.1 as f32 / (result.0 + result.1 + result.2) as f32 * 100.0,
        result.1
    );*/
}
