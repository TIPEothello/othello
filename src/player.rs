use parking_lot::Mutex;
use std::io::stdout;
use std::process::exit;

use crate::board::{Board, BoardState, Case, EndState};
use crate::mcts;
use crate::minimax::Tree;
use crossterm::cursor::MoveUp;

use rand::seq::SliceRandom;
use rayon::prelude::*;
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub enum Strategy {
    Random,
    Greedy,
    Manual,
    Minimax {
        depth: u8,
    },
    MCTS {
        playout_budget: usize,
        final_solve: bool,
    },
}

#[allow(unused)]
pub enum PlayStyle {
    Automatic,
    Progressive,
}

pub struct Player {
    strategy: (Strategy, Strategy),
}

impl Player {
    /// New players (Black,white)
    pub fn new(strategy: (Strategy, Strategy)) -> Self {
        Player { strategy }
    }
    #[allow(dead_code)]
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
        println!("{}", board);
        loop {
            let (current_player, other, turn) = match board.get_turn() {
                Case::Black => (&mut player1, &mut player2, 0),
                Case::White => (&mut player2, &mut player1, 1),
                Case::Empty => unreachable!(),
            };

            let move_ = current_player.get_move(&board);

            let state = board.play_move(&move_).unwrap();

            println!(
                "{:?} played {}",
                if turn == 0 {
                    self.strategy.0
                } else {
                    self.strategy.1
                },
                print_coords(&move_)
            );
            println!("{}", board);

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
        }
    }

    #[allow(dead_code)]
    pub fn play_games(&mut self, n: u32, verbose: bool, length: usize) -> (u32, u32, u32) {
        let score: Mutex<(u32, u32, u32)> = Mutex::new((0, 0, 0));
        if verbose {
            display_score(
                *score.lock(),
                n,
                length,
                (&self.strategy.0, &self.strategy.1),
            );
        }
        (0..n).into_par_iter().for_each(|_| {
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

                let state = board.play_move(&move_).unwrap();

                if state == BoardState::Ongoing {
                    current_player.update_board(&board);
                    other.update_board(&board);
                } else if let BoardState::Ended(end_state) = state {
                    let mut locked = score.lock();
                    match end_state {
                        EndState::Winner(Case::Black) => locked.0 += 1,
                        EndState::Winner(Case::White) => locked.1 += 1,
                        EndState::Winner(Case::Empty) => locked.2 += 1,
                    }
                    if verbose {
                        go_3_lines_up();
                        display_score(*locked, n, length, (&self.strategy.0, &self.strategy.1));
                        break;
                    }
                }
            }
        });
        let games_result = *score.lock();

        games_result
    }
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
        Strategy::MCTS {
            playout_budget,
            final_solve,
        } => Box::new(MctsPlayerAPI::new(
            playout_budget,
            final_solve,
            player,
            board,
        )),
    }
}

struct MctsPlayerAPI(mcts::MCTS);

impl MctsPlayerAPI {
    #[inline]
    fn new(playout_budget: usize, final_solve: bool, player: Case, board: &Board) -> Self {
        Self(mcts::MCTS::new(
            player,
            final_solve,
            playout_budget,
            board.clone(),
        ))
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

struct MinimaxPlayerAPI(Tree, u8);

impl MinimaxPlayerAPI {
    #[inline]
    fn new(depth: u8, board: &Board) -> Self {
        Self(Tree::from_board(&mut board.clone(), None, depth), depth)
    }
}

impl PlayerApiTrait for MinimaxPlayerAPI {
    #[inline]
    fn update_board(&mut self, board: &Board) {
        if self.1 <= 2 {
            self.0 = Tree::from_board(&mut board.clone(), None, self.1);
        }
    }
    #[inline]
    fn get_move(&mut self, board: &Board) -> (usize, usize) {
        if self.1 > 2 {
            if let Some(m) = board.history.moves.last() {
                for i in 0..self.0.moves {
                    if self.0.subtree.as_ref().unwrap()[i].mov.unwrap() == *m {
                        let subt = self.0.subtree.as_ref().unwrap()[i].clone();
                        self.0.subtree = subt.subtree;
                        self.0.moves = subt.moves;
                        self.0.cases = subt.cases;
                        self.0.score = subt.score;
                        self.0.value = subt.value;
                        self.0.mov = subt.mov;
                        break;
                    }
                }
            }
        }
        self.0.best_move(board.get_turn(), board, self.1)
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

fn display_score(score: (u32, u32, u32), n: u32, length: usize, strategy: (&Strategy, &Strategy)) {
    let (black, white, draw) = score;
    let l = length as f32;
    let mut black_label = format!("Black ({:?}) :", strategy.0,);
    let mut white_label = format!("White ({:?}):", strategy.1,);
    let mut draw_label = "Draw:".to_string();
    let longest = black_label
        .len()
        .max(white_label.len())
        .max(draw_label.len());
    black_label.push_str(" ".repeat(longest - black_label.len()).as_str());
    white_label.push_str(" ".repeat(longest - white_label.len()).as_str());
    draw_label.push_str(" ".repeat(longest - draw_label.len()).as_str());
    let black_p = black as f32 / n as f32 * 100.0;
    let white_p = white as f32 / n as f32 * 100.0;
    let draw_p = draw as f32 / n as f32 * 100.0;
    let black_pixel = black as f32 / n as f32 * l;
    let white_pixel = white as f32 / n as f32 * l;
    let draw_pixel = draw as f32 / n as f32 * l;

    let black_score =
        "▮".repeat(black_pixel as usize) + " ".repeat(length - black_pixel as usize).as_str();

    let white_score =
        "▮".repeat(white_pixel as usize) + " ".repeat(length - white_pixel as usize).as_str();
    let draw_score =
        "▮".repeat(draw_pixel as usize) + " ".repeat(length - draw_pixel as usize).as_str();
    println!(
        "{black_label} [{}] {black_p:.2} %\n{white_label} [{}] {white_p:.2} %\n{draw_label} [{}] {draw_p:.2} %",
        black_score, white_score, draw_score
    );
}

#[inline]
fn go_3_lines_up() {
    crossterm::queue!(stdout(), MoveUp(3)).unwrap();
}
