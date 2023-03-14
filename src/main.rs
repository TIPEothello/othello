mod board;
mod rules;

use board::Board;

fn main() {
	let mut board = Board::new();
	println!("{}", board);
	println!("{:?}", board.available_moves(board::Case::Black));
	board.make_move((2, 3), board::Case::Black).unwrap();
	println!("{}", board);
	println!("{:?}", board.available_moves(board::Case::White));
}