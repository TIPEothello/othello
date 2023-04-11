/*
 File: main.rs
 Created Date: 21 Mar 2023
 Author: realbacon
 -----
 Last Modified: 11/04/2023 01:20:47
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

mod board;
mod minimax;
mod player;
mod rules;

#[tokio::main(worker_threads = 10)]
async fn main() {
    let mut player = player::Player::new(Some((
        player::Strategy::Minimax { depth: 5 },
        player::Strategy::Random,
    )));
    println!("{:?}", player.play_games(100).await);
    let mut board = board::Board::new();
    //let tree = minimax::Tree::from_board(&mut board, None, 6);
    //println!("{:?}", tree);
}
