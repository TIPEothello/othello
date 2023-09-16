/*
 File: player.rs
 Created Date: 21 Mar 2023
 Author: realbacon
 -----
 Last Modified: 27/06/2023 01:03:56
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/
#![allow(dead_code, unused_imports)]

use std::cmp::Ordering;
use std::io::stdout;

use crate::board::{Board, BoardState, Case, EndState};
use crate::mcts;
use crate::minimax::{self, Tree};
use crossterm::cursor::MoveDown;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::QueueableCommand;
use rand::seq::SliceRandom;
use rayon::prelude::*;
#[derive(Debug, Clone, Copy)]
pub enum Strategy {
    Random,
    Greedy,
    Manual,
    Minimax { depth: u8 },
    MCTS { playout_budget: usize },
}

pub enum PlayStyle {
    Automatic,
    Progressive,
}

pub struct Player {
    board: Board,
    strategy: (Strategy, Strategy),
}

impl Player {
    /// New players (Black,white)
    pub fn new(strategy: (Strategy, Strategy)) -> Self {
        Player {
            board: Board::new(),
            strategy,
        }
    }

    pub fn progressive_play(&mut self) {
        let is_win = cfg!(windows);
        println!("Running on Windows: {}", is_win);
        println!("Welcome to Reversi (Othello)! - Rust Edition");

        let mut board = Board::new();
        let mut player1 =
            PlayerAPI::new(self.strategy.0, PlayStyle::Progressive, Case::Black, &board);
        let mut player2 =
            PlayerAPI::new(self.strategy.1, PlayStyle::Progressive, Case::White, &board);
        println!();
        loop {
            println!("{}", board);
            let (current_player, other) = match board.get_turn() {
                Case::Black => (&mut player1, &mut player2),
                Case::White => (&mut player2, &mut player1),
                Case::Empty => panic!("wtf bro"),
            };

            let (move_, mut quit) = current_player.get_move(&board);
            if quit {
                println!("Quitting...");
                return;
            }

            let state = board.play_move(&move_).unwrap();

            match state {
                BoardState::Ongoing => {
                    current_player.update_board(&board);
                    other.update_board(&board);
                }
                BoardState::Ended(end_state) => {
                    let (white, black) = board.score();
                    println!("White: {}, Black: {}", white, black);
                    println!(
                        "Winner: {}",
                        match end_state {
                            EndState::Winner(Case::Black) => "Black",
                            EndState::Winner(Case::White) => "White",
                            EndState::Winner(Case::Empty) => "Draw",
                        }
                    );
                    break;
                }
            }

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
                println!("Quitting...");
                return;
            }
        }
    }

    pub async fn play_games(&mut self, n: u32) -> (u32, u32, u32) {
        use tokio::task::spawn;
        let mut games_result = (0, 0, 0); // Black White Draw

        let games_result_par = (0..n)
            .into_par_iter()
            .map(|i| {
                let mut board = Board::new();
                let mut player1 = PlayerAPI::new(
                    self.strategy.0,
                    PlayStyle::Automatic,
                    Case::Black,
                    &board.clone(),
                );
                let mut player2 = PlayerAPI::new(
                    self.strategy.1,
                    PlayStyle::Automatic,
                    Case::White,
                    &board.clone(),
                );
                loop {
                    let (current_player, other) = match board.get_turn() {
                        Case::Black => (&mut player1, &mut player2),
                        Case::White => (&mut player2, &mut player1),
                        Case::Empty => panic!(""),
                    };

                    let move_ = current_player.get_move(&board).0;

                    //println!("{:?}", move_);

                    let state = board.play_move(&move_).unwrap();

                    if state == BoardState::Ongoing {
                        current_player.update_board(&board);
                        other.update_board(&board);
                    } else if let BoardState::Ended(end_state) = state {
                        println!("finished game {i}");
                        return end_state;
                    }
                }
            })
            .collect::<Vec<_>>();

        games_result.0 = games_result_par
            .iter()
            .filter(|end_state| matches!(end_state, EndState::Winner(Case::Black)))
            .count() as u32;
        games_result.1 = games_result_par
            .iter()
            .filter(|end_state| matches!(end_state, EndState::Winner(Case::White)))
            .count() as u32;

        games_result.2 = games_result_par
            .iter()
            .filter(|end_state| matches!(end_state, EndState::Winner(Case::Empty)))
            .count() as u32;
        games_result
    }
}

pub struct PlayerAPI {
    mcts: mcts::MCTS,
    minimax: Tree,
    strategy: Strategy,
    playstyle: PlayStyle,
    rng: rand::rngs::ThreadRng,
}

impl PlayerAPI {
    fn new(strategy: Strategy, playstyle: PlayStyle, player: Case, board: &Board) -> Self {
        let (minimax, mcts) = match strategy {
            Strategy::Greedy | Strategy::Manual | Strategy::Random => (
                Tree::from_board(&mut board.clone(), None, 1),
                mcts::MCTS::new(player, 0, board.clone()),
            ),
            Strategy::MCTS { playout_budget } => (
                Tree::from_board(&mut board.clone(), None, 1),
                mcts::MCTS::new(player, playout_budget, board.clone()),
            ),
            Strategy::Minimax { depth } => (
                Tree::from_board(&mut board.clone(), None, depth),
                mcts::MCTS::new(player, 0, board.clone()),
            ),
        };

        Self {
            mcts,
            minimax,
            strategy,
            playstyle,
            rng: rand::thread_rng(),
        }
    }

    fn update_board(&mut self, board: &Board) {
        if let Strategy::Minimax { depth } = self.strategy {
            self.minimax = Tree::from_board(&mut board.clone(), None, depth);
        };
    }

    fn get_move(&mut self, board: &Board) -> ((usize, usize), bool) {
        match self.strategy {
            Strategy::Random => (
                *board.available_moves(None).choose(&mut self.rng).unwrap(),
                false,
            ),
            Strategy::Greedy => (board.move_with_highest_gain().unwrap(), false),
            Strategy::MCTS { .. } => (self.mcts.search(board), false),
            Strategy::Minimax { .. } => (self.minimax.best_move(board.get_turn()), false),
            Strategy::Manual => {
                if matches!(self.playstyle, PlayStyle::Automatic) {
                    panic!("Manual is not supported in automatic games !");
                }
                let moves = board.available_moves(None);
                println!(
                    "Available moves: {}",
                    moves
                        .iter()
                        .map(print_coords)
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                loop {
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
                            break ((0, 0), true);
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
                    break (input, false);
                }
            }
        }
    }
}

pub fn print_coords(c: &(usize, usize)) -> String {
    // A1
    let x = c.0 as u8 + 65;
    let y = c.1 as u8 + 49;
    format!("{}{}", x as char, y as char)
}
