/*
 File: main.rs
 Created Date: 14 Mar 2023
 Author: realbacon
 -----
 Last Modified: 15/03/2023 03:39:24
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

mod board;
mod rules;
use board::{Board, Case};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::{cursor::MoveDown, cursor::MoveUp, QueueableCommand};
use rand::seq::SliceRandom;
use rules::enemy;
use std::io::stdout;

fn main() {
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
