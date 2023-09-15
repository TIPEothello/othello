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

    /*pub async fn play_games(&mut self, n: u32) -> (u32, u32, u32) {
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
                        /*Strategy::MCTS { depth } => {
                            let mut mcts = mcts::MCTS::new(board.clone());
                            mcts.expand_by_depth(depth);
                            let bmov = mcts.best_move(100, Case::Black);
                            board.play_move(&bmov).unwrap();
                        }*/
                        Strategy::Random => {
                            let bmove = *board.available_moves(None).choose(&mut rng).unwrap();
                            board.play_move(&bmove).unwrap();
                        }
                        Strategy::Greedy => {
                            board.play_move_with_highest_gain().unwrap();
                        }
                        Strategy::MinimaxTree { depth } => {
                            let mut tree = minimax::Tree::from_board(&mut board, None, depth);
                            let best_tree = minimax::minimax_tree(&mut tree, board.get_turn());
                            board.play_move(&best_tree.mov.unwrap()).unwrap();
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
                }
                Ordering::Less => {
                    games_result.1 += 1;
                }
                Ordering::Equal => {
                    games_result.2 += 1;
                }
            }
        }
        games_result
    }*/
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
