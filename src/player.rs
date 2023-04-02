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
use crate::minimax;
use crate::rules::enemy;

pub enum Strategy {
    Random,
    Greedy,
    Minimax { depth: i8 },
}

pub struct Player {
    board: Board,
    strategy: (Strategy, Strategy),
}

impl Player {
    pub fn new(strategy: Option<(Strategy, Strategy)>) -> Player {
        let strategy = match strategy {
            Some(s) => s,
            None => (Strategy::Random, Strategy::Random),
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
        let mut stdout = stdout();
        let mut quit = false;
        println!();
        while !board.available_moves(None).is_empty() {
            println!("{}", board);
            let strategy = match board.get_turn() {
                Case::White => &self.strategy.1,
                Case::Black => &self.strategy.0,
                Case::Empty => {
                    panic!("Empty case is not a valid turn");
                }
            };
            match strategy {
                Strategy::Random => {
                    let bmove = *board.available_moves(None).choose(&mut rng).unwrap();
                    board.make_move(&bmove).unwrap();
                }
                Strategy::Greedy => {
                    board.make_move_with_highest_gain().unwrap();
                }
                Strategy::Minimax { depth } => {
                    let outcomes = minimax::calculate_outcomes(&board, *depth);
                    let best_move = minimax::minimax(&outcomes, &mut board);
                    board.make_move(&best_move).unwrap();
                }
            }
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
            while !board.available_moves(None).is_empty() {
                let strategy = match board.get_turn() {
                    Case::White => &self.strategy.1,
                    Case::Black => &self.strategy.0,
                    Case::Empty => {
                        panic!("Empty case is not a valid turn");
                    }
                };
                match strategy {
                    Strategy::Random => {
                        let bmove = *board.available_moves(None).choose(&mut rng).unwrap();
                        board.make_move(&bmove).unwrap();
                    },
                    Strategy::Greedy => {
                        board.make_move_with_highest_gain().unwrap();
                    }
                    Strategy::Minimax { depth } => {
						println!("{}", board.history.len());
                        let outcomes = minimax::calculate_outcomes(&board, *depth);
						println!("{}", board.history.len());
                        let best_move = minimax::minimax(&outcomes, &mut board);
						println!("{}", board.history.len());
                        board.make_move(&best_move).unwrap();
                    }
                }
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
        (black_wins, white_wins, draws)
    }
}
