/*
 File: player.rs
 Created Date: 21 Mar 2023
 Author: realbacon
 -----
 Last Modified: 2/04/2023 12:18:30
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/
#![allow(dead_code)]

use std::cmp::Ordering;
use std::io::stdout;

use crossterm::cursor::{MoveDown, MoveUp};
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::QueueableCommand;
use rand::seq::SliceRandom;

use crate::board::{Board, Case};
use crate::rules::enemy;

pub enum Strategy {
    Random,
    Mixed,
    Greedy,
}

pub struct Player {
    board: Board,
    strategy: Strategy,
}

impl Player {
    pub fn new(strategy: Option<Strategy>) -> Player {
        let strategy = match strategy {
            Some(s) => s,
            None => Strategy::Random,
        };

        Player {
            board: Board::new(),
            strategy,
        }
    }

    pub fn progressive_play(&mut self) {
        let is_win = cfg!(windows);
        println!("Running on Windows: {}", is_win);
        println!("Welcome to Reversi (Othello)! - Rust Edition");
        let mut rng = rand::thread_rng();
        let mut board = Board::new();
        let mut turn = Case::Black;
        let mut stdout = stdout();
        let mut quit = false;
        println!();
        while !board.available_moves(Some(turn)).is_empty() {
            println!("{}", board);

            match self.strategy {
                Strategy::Random => {
                    let bmove = *board.available_moves(Some(turn)).choose(&mut rng).unwrap();
                    board.make_move(&bmove).unwrap();
                }
                Strategy::Mixed => match turn {
                    Case::White => {
                        let bmove = *board.available_moves(Some(turn)).choose(&mut rng).unwrap();
                        board.make_move(&bmove).unwrap();
                    }
                    Case::Black => {
                        board.make_move_with_highest_gain().unwrap();
                    }
                    _ => {}
                },
                Strategy::Greedy => {
                    board.make_move_with_highest_gain().unwrap();
                }
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
            match white.cmp(&black) {
                Ordering::Greater => {
                    "White"
                }
                Ordering::Less => {
                    "Black"
                }
                Ordering::Equal => {
                    "Draw"
                }
            }
        );
    }

    pub fn play_games(&mut self, n: u32) -> (u32, u32, u32) {
        let mut rng = rand::thread_rng();
        let mut white_wins = 0;
        let mut black_wins = 0;
        let mut draws = 0;
        for _ in 0..n {
            let mut board = Board::new();
            let mut turn = Case::Black;
            while !board.available_moves(Some(turn)).is_empty() {
                match self.strategy {
                    Strategy::Random => {
                        let bmove = *board.available_moves(Some(turn)).choose(&mut rng).unwrap();
                        board.make_move(&bmove).unwrap();
                    }
                    Strategy::Mixed => match turn {
                        Case::White => {
                            let bmove =
                                *board.available_moves(Some(turn)).choose(&mut rng).unwrap();
                            board.make_move(&bmove).unwrap();
                        }
                        Case::Black => {
                            board.make_move_with_highest_gain().unwrap();
                        }
                        _ => {}
                    },
                    Strategy::Greedy => {
                        board.make_move_with_highest_gain().unwrap();
                    }
                }
                turn = enemy(&turn);
            }
            let (white, black) = board.score();
            match white.cmp(&black) {
                Ordering::Equal => {
                    draws += 1;
                }
                Ordering::Greater => {
                    white_wins += 1;
                }
                Ordering::Less => {
                    black_wins += 1;
                }
            }
        }
        (white_wins, black_wins, draws)
    }
}
