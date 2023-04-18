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
    let mut board = board::Board::new();
	for _ in 0..4 {
		board.make_move(&board.available_moves(None)[0]).unwrap();
	}
	let mut tree = minimax::Tree::from_board(&mut board.clone(), None, 4);
	minimax::minimax_tree(&mut tree, board::Case::Black);

}
