/*
 File: main.rs
 Created Date: 14 Mar 2023
 Author: realbacon
 -----
 Last Modified: 14/03/2023 11:10:46
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

mod board;
mod rules;
use board::{Board, Case};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

use rand::seq::SliceRandom;
use rules::enemy;

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
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => break,
                _ => (),
            }
        }
    }

    println!("{}", board);

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
