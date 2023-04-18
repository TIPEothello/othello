/*
 File: main.rs
 Created Date: 21 Mar 2023
 Author: realbacon
 -----
 Last Modified: 18/04/2023 08:19:33
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

mod board;
mod minimax;
mod player;
mod rules;

#[tokio::main(worker_threads = 100)]
async fn main() {
    let mut player = player::Player::new(Some((
        player::Strategy::MinimaxTree { depth: 4 },
        player::Strategy::MinimaxTree { depth: 4 },
    )));
    let result = player.play_games(100).await;
    println!(
        "Win ratio : White {}% ({}) - Black {}% ({})",
        result.0 as f32 / (result.0 + result.1 + result.2) as f32 * 100.0,
        result.0,
        result.1 as f32 / (result.0 + result.1 + result.2) as f32 * 100.0,
        result.1
    );
}
