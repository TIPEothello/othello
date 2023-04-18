/*
 File: main.rs
 Created Date: 21 Mar 2023
 Author: realbacon
 -----
 Last Modified: 18/04/2023 07:24:41
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
    let board = board::Board::new();
	let mut tree = minimax::Tree::from_board(&mut board.clone(), None, 6);
	minimax::minimax_tree(&mut tree, board::Case::Black);

}
