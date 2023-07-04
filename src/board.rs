/*
 File: board.rs
 Created Date: 21 Mar 2023
 Author: realbacon
 -----
 Last Modified: 27/06/2023 02:13:20
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/
#![allow(dead_code)]

use crate::rules::{check_direction, is_legal_move, is_legal_move_with_gain};
use ansi_term::{Colour, Colour::*, Style};
use std::fmt::Display;
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Case {
    Empty,
    White,
    Black,
}

impl Case {
    pub fn opponent(&self) -> Case {
        match self {
            Case::Empty => Case::Empty,
            Case::White => Case::Black,
            Case::Black => Case::White,
        }
    }
}

#[derive(Debug, Clone)]
pub struct History {
    pub moves: Vec<(usize, usize)>,
    pub history: Vec<[[Case; 8]; 8]>,
}

impl Display for Case {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Case::Empty => write!(f, "Empty"),
            Case::White => write!(f, "White"),
            Case::Black => write!(f, "Black"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    pub cases: [[Case; 8]; 8],
    pub history: History,
}
pub const DIRECTIONS: [(i8, i8); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

const MID: [(usize, usize); 4] = [(3, 3), (3, 4), (4, 3), (4, 4)];

impl Board {
    /// Create a new board
    /// # Returns
    /// * A new board with start cases filled in
    pub fn new() -> Self {
        let mut board = Board {
            cases: [[Case::Empty; 8]; 8],
            history: History {
                moves: Vec::<(usize, usize)>::with_capacity(60),
                history: Vec::<[[Case; 8]; 8]>::with_capacity(60),
            },
        };
        board.cases[3][3] = Case::White;
        board.cases[4][4] = Case::White;
        board.cases[3][4] = Case::Black;
        board.cases[4][3] = Case::Black;
        board
    }

    /// Create a board from a 2d-array of cases (8x8)
    /// # Returns
    /// * Board
    pub fn from_cases(cases: [[Case; 8]; 8]) -> Self {
        Board {
            cases,
            history: History {
                moves: {
                    let mut v = Vec::<(usize, usize)>::with_capacity(60);
                    for i in 0..8 {
                        for j in 0..8 {
                            if cases[i][j] != Case::Empty && !MID.contains(&(i, j)) {
                                v.push((i, j));
                            }
                        }
                    }
                    v
                },
                history: Vec::<[[Case; 8]; 8]>::with_capacity(60),
            },
        }
    }

    pub fn from_moves(moves: &[(usize, usize)]) -> Self {
        let mut board = Board::new();
        board.play_moves(moves).unwrap();
        return board;
    }

    /// Get the current turn
    pub fn get_turn(&self) -> Case {
        let sum = self.score().0 + self.score().1;
		return if sum % 2 == 0 {
			Case::Black
		} else {
			Case::White
		}
    }

    /// Play a move on the board
    /// # Arguments
    /// * `bmove` - The move to make
    /// # Returns
    /// * `Ok(())` if the move is legal
    /// * `Err(String)` if the move is illegal
    pub fn play_move(&mut self, bmove: &(usize, usize)) -> Result<(), String> {
        let color = self.get_turn();
        if !is_legal_move(&self.cases, *bmove, &color) {
            let mut s = String::new();
            s.push_str("Illegal move : ");
            s.push_str(&format!("{:?}", bmove));

            return Err(s);
        }
        self.cases[bmove.0][bmove.1] = color;

        for direction in DIRECTIONS {
            if check_direction(
                &self.cases,
                (bmove.0 as i8, bmove.1 as i8),
                direction,
                &color,
            ) {
                let mut x = bmove.0 as i8 + direction.0;
                let mut y = bmove.1 as i8 + direction.1;
                while self.cases[x as usize][y as usize] == (&color).opponent() {
                    self.cases[x as usize][y as usize] = color;
                    x += direction.0;
                    y += direction.1;
                }
            }
        }
        self.history.moves.push(*bmove);
        self.history.history.push(self.cases);
        Ok(())
    }

    pub fn play_move_with_highest_gain(&mut self) -> Result<(), String> {
        let moves = self.available_moves_with_gain();
        if moves.is_empty() {
            return Err("No moves available".to_string());
        }
        let mut highest_gain = 0;
        let mut highest_move = (0, 0);
        for (m, g) in moves {
            if g > highest_gain {
                highest_gain = g;
                highest_move = m;
            }
        }
        self.play_move(&highest_move)
    }

    /// Returns a vector of all the available moves for a given color
    /// # Arguments
    /// * `color` - The color of the player
    pub fn available_moves(&self, color: Option<Case>) -> Vec<(usize, usize)> {
        let current_color = self.get_turn();
        let color = color.unwrap_or(current_color);
        let mut moves = Vec::new();
        for i in 0..8 {
            for j in 0..8 {
                if is_legal_move(&self.cases, (i, j), &color) {
                    moves.push((i, j));
                }
            }
        }
        moves
    }

    pub fn available_moves_with_gain(&self) -> Vec<((usize, usize), usize)> {
        let mut moves = Vec::new();
        let color = self.get_turn();
        for i in 0..8 {
            for j in 0..8 {
                let (legal, gain) = is_legal_move_with_gain(&self.cases, (i, j), &color);
                if legal {
                    moves.push(((i, j), gain));
                }
            }
        }
        moves
    }

    /// Returns the score of the board (whiet,black)
    pub fn score(&self) -> (usize, usize) {
        let mut white = 0;
        let mut black = 0;
        for line in self.cases.iter() {
            for case in line.iter() {
                match case {
                    Case::White => white += 1,
                    Case::Black => black += 1,
                    Case::Empty => (),
                }
            }
        }
        (white, black)
    }

    pub fn reset(&mut self, num: usize) {
        if num == 0 || self.history.history.is_empty() {
            return;
        }
        if num >= self.history.history.len() {
            self.cases = Board::new().cases;
            self.history.history.clear();
            self.history.moves.clear();
            return;
        }
        self.cases = self.history.history[self.history.history.len() - num - 1];
        // Remove the last num moves
        self.history
            .history
            .truncate(self.history.history.len() - num);
        self.history.moves.truncate(self.history.moves.len() - num);
    }

    pub fn play_moves(&mut self, moves: &[(usize, usize)]) -> Result<(), &str> {
        for m in moves.iter() {
            let mover = self.play_move(m);
            if mover.is_err() {
                println!("Error: {:?} \n{}", moves, self);
                panic!("");
            }
        }
        Ok(())
    }

    pub fn to_u64(&self) -> (u64, u64) {
        let mut res1: u64 = 0;
        let mut res2: u64 = 0;
        for i in 0..8 {
            for j in 0..8 {
                res1 |= (self.cases[i][j] != Case::Empty) as u64;
                res2 |= (self.cases[i][j] == Case::Black) as u64;
                if i == j && j == 7 {
                    continue;
                }
                res1 <<= 1;
                res2 <<= 1;
            }
        }
        (res1, res2)
    }

    pub fn from_u64(input: (u64, u64)) -> Self {
        let mut cases: [[Case; 8]; 8] = [[Case::Empty; 8]; 8];
        for i in 0..8 {
            for j in 0..8 {
                let k = (1_u64 << (64 - 8 * i - j - 1)) as u64;
                if (input.0 & k) != 0 {
                    cases[i][j] = if (input.1 & k) != 0 {
                        Case::Black
                    } else {
                        Case::White
                    }
                }
            }
        }
        return Board::from_cases(cases);
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn change_color(bg: &mut u8) -> Colour {
            if bg == &1 {
                *bg = 0;
                RGB(15, 117, 70)
            } else {
                *bg = 1;
                RGB(17, 153, 90)
            }
        }
        let mut string = String::from("   1  2  3  4  5  6  7  8 \n");
        let mut bg: u8 = 0;
        for (i, line) in self.cases.iter().enumerate() {
            string.push_str(&format!("{} ", (i as u8 + 65) as char)[..]);
            for case in line.iter() {
                match case {
                    Case::Empty => {
                        string.push_str(
                            Style::new()
                                .on(change_color(&mut bg))
                                .paint("ㅤ ")
                                .to_string()
                                .as_str(),
                        );
                    }
                    Case::White => {
                        string.push_str(
                            Style::new()
                                .on(change_color(&mut bg))
                                .bold()
                                .fg(RGB(255, 255, 255))
                                .paint(" ⬤ ")
                                .to_string()
                                .as_str(),
                        );
                    }
                    Case::Black => {
                        string.push_str(
                            Style::new()
                                .on(change_color(&mut bg))
                                .bold()
                                .fg(RGB(0, 0, 0))
                                .paint(" ⬤ ")
                                .to_string()
                                .as_str(),
                        );
                    }
                }
            }
            change_color(&mut bg);
            string += "\n";
        }
        write!(f, "{}", string)
    }
}

#[test]
fn test_u64() {
    let board = Board::new();
    let value = board.to_u64();
    assert_eq!(value.0, 0x0000001818000000);
    assert_eq!(value.1, 0x0000000810000000);
    let remade = Board::from_u64((0x000f001818000000, 0x000a000810000000));
    println!("{}", remade);
}

#[test]
fn play_move_test() {
    let mut board = Board::new();
    board
        .play_move(&(0, 0))
        .expect_err("Move should not be legal");
    assert_eq!(board.cases[0][0], Case::Empty); // Check that the move was not made
                                                // Check the initial board
    assert_eq!(board.cases[3][3], Case::White);
    assert_eq!(board.cases[4][4], Case::White);
    assert_eq!(board.cases[3][4], Case::Black);
    assert_eq!(board.cases[4][3], Case::Black);
    println!("{}", board);
}

#[test]
fn available_moves_test() {
    let board = Board::new();
    let moves = board.available_moves(None);
    assert_eq!(moves.len(), 4);
    assert!(moves.contains(&(2, 3)));
    assert!(moves.contains(&(3, 2)));
    assert!(moves.contains(&(4, 5)));
    assert!(moves.contains(&(5, 4)));
}

#[test]
fn reset_test() {
    let mut board = Board::new();
    board
        .play_move(&board.available_moves(None)[0])
        .expect("Move should be legal");
    //println!("{}", board);
    board.reset(1);
    //println!("{}", board);
    assert_eq!(board.cases[3][3], Case::White);
    assert_eq!(board.cases[4][4], Case::White);
    assert_eq!(board.cases[3][4], Case::Black);
    assert_eq!(board.cases[4][3], Case::Black);
}

pub struct Move {
    pub move_: (usize, usize),
}

impl Move {
    pub fn new(move_: (usize, usize)) -> Self {
        Self { move_ }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            self.move_.1 + 1,
            (self.move_.0 as u8 + 97) as char,
        )
    }
}
