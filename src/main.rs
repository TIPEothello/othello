mod board;
mod rules;
use board::{Board, Case};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::{cursor::MoveDown, cursor::MoveUp, QueueableCommand};
use rand::seq::SliceRandom;
use rules::enemy;
use std::io::stdout;

fn progressive_play() {
    let is_win = cfg!(windows);
    println!("Running on Windows: {}", is_win);
    println!("Welcome to Reversi (Othello)! - Rust Edition");
    let mut rng = rand::thread_rng();
    let mut board = Board::new();
    let mut turn = Case::Black;
    let mut stdout = stdout();
    let mut quit = false;
    println!("");
    while board.available_moves(&turn).len() > 0 {
        println!("{}", board);

        match turn {
            Case::White => {
                let bmove = *board.available_moves(&turn).choose(&mut rng).unwrap();
                board.make_move(bmove, &turn).unwrap();
            }
            Case::Black => {
                board.make_move_with_highest_gain(&turn).unwrap();
            },
			_ => {}
        }
        // choose a random move within the available moves

        turn = enemy(&turn);
        // Wait for user to press enter

        loop {
            match read().unwrap() {
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => break,
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => {
                    quit = true;
                    break;
                }
                _ => {}
            }
        }
        if quit {
            break;
        }
        stdout.queue(MoveUp(if is_win { 10 } else { 11 })).unwrap();
    }

    if quit {
        println!("Quitting...");
        return;
    }

    println!("{}", board);
    stdout.queue(MoveDown(10)).unwrap();
    let (white, black) = board.score();
    println!("White: {}, Black: {}", white, black);
    println!(
        "Winner: {}",
        if white > black {
            "White"
        } else if black > white {
            "Black"
        } else {
            "Draw"
        }
    );
}


fn play_games(n: u32) -> (u32, u32, u32) {
	let mut rng = rand::thread_rng();
	let mut white_wins = 0;
	let mut black_wins = 0;
	let mut draws = 0;
	for _ in 0..n {
		let mut board = Board::new();
		let mut turn = Case::Black;
		while board.available_moves(&turn).len() > 0 {
			match turn {
				Case::White => {
					let bmove = *board.available_moves(&turn).choose(&mut rng).unwrap();
					board.make_move(bmove, &turn).unwrap();
				}
				Case::Black => {
					board.make_move_with_highest_gain(&turn).unwrap();
				},
				_ => {}
			}
			turn = enemy(&turn);
		}
		let (white, black) = board.score();
		if white > black {
			white_wins += 1;
		} else if black > white {
			black_wins += 1;
		} else {
			draws += 1;
		}
	}
	(white_wins, black_wins, draws)
}

fn main() {
	/*let (white_wins, black_wins, draws) = play_games(15000);
	println!("White wins: {}, Black wins: {}, Draws: {}", white_wins, black_wins, draws);
	println!("Efficiency: {}%", (black_wins as f32) * 100.0 / (white_wins + black_wins + draws) as f32);*/
	progressive_play();
}