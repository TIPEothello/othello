mod board;
mod rules;

use board::Board;
use board::Case;
use rules::enemy;
use rand::seq::SliceRandom;

fn main() {
	let mut rng = rand::thread_rng();
	let mut board = Board::new();
	let mut turn = Case::Black;
	while board.available_moves(&turn).len() > 0 {
		println!("{}", board);
		// choose a random move within the available moves
		let bmove = *board.available_moves(&turn).choose(&mut rng).unwrap();
		board.make_move(bmove, turn).unwrap();
		turn = enemy(&turn);
	}

	println!("{}", board);
}