mod board;
mod rules;

use board::Board;
use board::Case;
use rules::enemy;
use rand::seq::SliceRandom;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers, KeyEventKind, KeyEventState};

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
		// Wait for user to press enter

		loop {
			match read().unwrap() {
				Event::Key(KeyEvent { code: KeyCode::Enter, modifiers: KeyModifiers::NONE, kind: KeyEventKind::Press, state: KeyEventState::NONE }) => break,
				_ => (),
			}
		}

	}

	println!("{}", board);

	let (white, black) = board.score();
	println!("White: {}, Black: {}", white, black);
	print!("Winner: {}" , if white > black { "White" } else if black > white { "Black" } else { "Draw" });
}