/*
 File: player.rs
 Created Date: 21 Mar 2023
 Author: realbacon
 -----
 Last Modified: 23/04/2023 05:12:9
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/
#![allow(dead_code)]

use std::cmp::Ordering;
use std::io::stdout;

use crate::board::{Board, Case};
use crate::minimax;
use crossterm::cursor::MoveDown;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::QueueableCommand;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, Copy)]
pub enum Strategy {
    Random,
    Greedy,
    Manual,
    Minimax { depth: u8 },
    MinimaxTree { depth: u8 },
}

pub struct Player {
    board: Board,
    strategy: (Strategy, Strategy),
}

impl Player {
    /// New players (Black,white)
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
            let mut auto = true;
            println!("{}", board);
            let strategy = match board.get_turn() {
                Case::White => &self.strategy.0,
                Case::Black => &self.strategy.1,
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
                Strategy::MinimaxTree { depth } => {
                    let mut tree = minimax::Tree::from_board(&mut board, None, *depth);
                    let best_tree = minimax::minimax_tree(&mut tree, board.get_turn());
                    board.make_move(&best_tree.mov.unwrap()).unwrap();
                }
                Strategy::Manual => {
                    auto = false;
                    let moves = board.available_moves(None);
                    if moves.is_empty() {
                        break;
                    }
                    println!(
                        "Available moves: {}",
                        moves
                            .iter()
                            .map(print_coords)
                            .collect::<Vec<String>>()
                            .join(", ")
                    );
                    // Get user input with std::io
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim();
                    let input = input
                        .split("")
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<&str>>();
                    // Input can be A1 or a1
                    if input.is_empty() {
                        continue;
                    }
                    if input.len() != 2 {
                        if input[0] == "q" {
                            quit = true;
                            break;
                        }
                        println!("Invalid move");
                        continue;
                    }
                    let input = (
                        input[0].to_uppercase().chars().next().unwrap() as usize - 65,
                        input[1].parse::<usize>().unwrap() - 1,
                    );
                    if !moves.contains(&input) {
                        println!("Invalid move");
                        continue;
                    }
                    board.make_move(&input).unwrap();
                }


            }
            // Wait for user to press enter

            if auto {
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
            }
            if quit {
                break;
            }
            //stdout.queue(MoveUp(if is_win { 10 } else { 11 })).unwrap();
            println!(
                "Playing move: {}",
                print_coords(board.history.moves.last().unwrap())
            );
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

    pub async fn play_games(&mut self, n: u32) -> (u32, u32, u32) {
        use rand::rngs::OsRng;
        use tokio::task::spawn;
        let mut games_result = (0, 0, 0); // White Black Draw

        let mut game_handler = Vec::new();
        let strat = self.strategy;
        let mut rng = OsRng::default();
        for _ in 0..n {
            let game_thread = spawn(async move {
                let mut board = Board::new();
                while !board.available_moves(None).is_empty() {
                    let strategy = match board.get_turn() {
                        Case::White => strat.0,
                        Case::Black => strat.1,
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
                            let outcomes = minimax::calculate_outcomes(&board, depth);
                            if outcomes.is_empty() {
                                break;
                            }
                            let best_move = minimax::minimax(&outcomes, &mut board);
                            board.make_move(&best_move).unwrap();
                        }
                        Strategy::MinimaxTree { depth } => {
                            let mut tree = minimax::Tree::from_board(&mut board, None, depth);
                            let best_tree = minimax::minimax_tree(&mut tree, board.get_turn());
                            board.make_move(&best_tree.mov.unwrap()).unwrap();
                        }
                        Strategy::Manual => {
                            panic!("Manual strategy is not supported in play_games");
                        }
                    }
                }
                board.score()
            });
            game_handler.push(game_thread);
        }
        for game in game_handler {
            let (white, black) = game.await.unwrap();
            match white.cmp(&black) {
                Ordering::Greater => {
                    games_result.0 += 1;
                },
                Ordering::Less => {
                    games_result.1 += 1;
                },
                Ordering::Equal => {
                    games_result.2 += 1;
                },
            }
        }
        games_result
    }
}

pub fn print_coords(c: &(usize, usize)) -> String {
    // A1
    let x = c.0 as u8 + 65;
    let y = c.1 as u8 + 49;
    format!("{}{}", x as char, y as char)
}
