/*
 File: main.rs
 Created Date: 21 Mar 2023
 Author: realbacon
 -----
 Last Modified: 18/04/2023 07:17:56
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
        player::Strategy::Random,
        player::Strategy::MinimaxTree { depth: 4 },
    )));
    println!("{:?}", player.progressive_play());
}
