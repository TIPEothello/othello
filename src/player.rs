#![allow(dead_code, unused_imports)]
use std::cmp::Ordering;
use std::io::stdout;
use std::process::{exit, ExitCode};
use std::sync::Mutex;

use crate::board::{self, Board, BoardState, Case, EndState};
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
            new_player_api(self.strategy.0, PlayStyle::Progressive, Case::Black, &board);
        let mut player2 =
            new_player_api(self.strategy.1, PlayStyle::Progressive, Case::White, &board);
        println!();
        loop {
            println!("{}", board);
            let mut quit = false;
            let (current_player, other) = match board.get_turn() {
                Case::Black => (&mut player1, &mut player2),
                Case::White => (&mut player2, &mut player1),
                Case::Empty => panic!("wtf bro"),
            };

            let move_ = current_player.get_move(&board);

            let state = board.play_move(&move_).unwrap();

            match state {
                BoardState::Ongoing => {
                    current_player.update_board(&board);
                    other.update_board(&board);
                }
                BoardState::Ended(end_state) => {
                    let (black, white) = board.score();
                    println!("Black: {}, White: {}", black, white);
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
        let vec: Mutex<(u32, u32, u32)> = Mutex::new((0, 0, 0));
        (0..n).into_par_iter().for_each(|i| {
            let mut board = Board::new();
            let mut player1 = new_player_api(
                self.strategy.0,
                PlayStyle::Automatic,
                Case::Black,
                &board.clone(),
            );
            let mut player2 = new_player_api(
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

                let move_ = current_player.get_move(&board);

                //println!("{:?}", move_);

                let state = board.play_move(&move_).unwrap();

                if state == BoardState::Ongoing {
                    current_player.update_board(&board);
                    other.update_board(&board);
                } else if let BoardState::Ended(end_state) = state {
                    println!("finished game {i}");
                    let mut locked = vec.lock().unwrap();
                    match end_state {
                        EndState::Winner(Case::Black) => locked.0 += 1,
                        EndState::Winner(Case::White) => locked.1 += 1,
                        EndState::Winner(Case::Empty) => locked.2 += 1,
                    }
                }
            }
        });
        let games_result = *vec.lock().unwrap();
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

trait PlayerApiTrait {
    fn update_board(&mut self, board: &Board);
    fn get_move(&mut self, board: &Board) -> (usize, usize);
}

fn new_player_api(
    strategy: Strategy,
    playstyle: PlayStyle,
    player: Case,
    board: &Board,
) -> Box<dyn PlayerApiTrait> {
    match strategy {
        Strategy::Random => Box::new(RandomPlayerAPI),
        Strategy::Greedy => Box::new(GreedyPlayerAPI),
        Strategy::Manual => {
            if matches!(playstyle, PlayStyle::Automatic) {
                panic!("Manual is not supported in automatic games !");
            }
            Box::new(ManualPlayerAPI)
        }
        Strategy::Minimax { depth } => Box::new(MinimaxPlayerAPI::new(depth, board)),
        Strategy::MCTS { playout_budget } => {
            Box::new(MctsPlayerAPI::new(playout_budget, player, board))
        }
    }
}

struct MctsPlayerAPI(mcts::MCTS);

impl MctsPlayerAPI {
    #[inline]
    fn new(playout_budget: usize, player: Case, board: &Board) -> Self {
        Self(mcts::MCTS::new(player, playout_budget, board.clone()))
    }
}

impl PlayerApiTrait for MctsPlayerAPI {
    #[inline]
    fn update_board(&mut self, _board: &Board) {}

    #[inline]
    fn get_move(&mut self, board: &Board) -> (usize, usize) {
        self.0.search(board)
    }
}

struct ManualPlayerAPI;

impl PlayerApiTrait for ManualPlayerAPI {
    #[inline]
    fn update_board(&mut self, _board: &Board) {}

    fn get_move(&mut self, board: &Board) -> (usize, usize) {
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
                    exit(130);
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
            break (input);
        }
    }
}

struct MinimaxPlayerAPI(Tree);

impl MinimaxPlayerAPI {
    #[inline]
    fn new(depth: u8, board: &Board) -> Self {
        Self(Tree::from_board(&mut board.clone(), None, depth))
    }
}

impl PlayerApiTrait for MinimaxPlayerAPI {
    #[inline]
    fn update_board(&mut self, board: &Board) {
        self.0 = Tree::from_board(&mut board.clone(), None, self.0.depth);
    }
    #[inline]
    fn get_move(&mut self, board: &Board) -> (usize, usize) {
        self.0.best_move(board.get_turn())
    }
}

struct RandomPlayerAPI;

impl PlayerApiTrait for RandomPlayerAPI {
    #[inline]
    fn update_board(&mut self, _board: &Board) {}

    #[inline]
    fn get_move(&mut self, board: &Board) -> (usize, usize) {
        *board
            .available_moves(None)
            .choose(&mut rand::thread_rng())
            .unwrap()
    }
}

struct GreedyPlayerAPI;

impl PlayerApiTrait for GreedyPlayerAPI {
    #[inline]
    fn update_board(&mut self, _board: &Board) {}

    #[inline]
    fn get_move(&mut self, board: &Board) -> (usize, usize) {
        board.move_with_highest_gain().unwrap()
    }
}

pub fn print_coords(c: &(usize, usize)) -> String {
    // A1
    let x = c.0 as u8 + 65;
    let y = c.1 as u8 + 49;
    format!("{}{}", x as char, y as char)
}

fn display_score(score: (u32, u32, u32)) {}
