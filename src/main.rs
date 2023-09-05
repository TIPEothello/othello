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

#[tokio::main(worker_threads = 10, flavor = "multi_thread")]
async fn main() {
    /*let mut player = player::Player::new(Some((
        player::Strategy::MinimaxTree { depth: 6 },
        player::Strategy::Manual,
    )));
    player.progressive_play();
    let result = player.play_games(100).await;
    println!(
        "Win ratio : White {}% ({}) - Black {}% ({})",
        result.0 as f32 / (result.0 + result.1 + result.2) as f32 * 100.0,
        result.0,
        result.1 as f32 / (result.0 + result.1 + result.2) as f32 * 100.0,
        result.1
    );*/
    use mcts::test_mcts;
    test_mcts();
}
