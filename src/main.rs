mod board;
mod rules;

use board::Board;

fn main() {
	let mut board = Board::new();
	println!("{}", board);
	println!("{:?}", board.available_moves(board::Case::White));
	board.make_move((2, 2), board::Case::Black).unwrap();
	println!("{}", board);
}