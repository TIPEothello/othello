/*
 File: board.rs
 Created Date: 14 Mar 2023
 Author: realbacon
 -----
 Last Modified: 15/03/2023 03:39:42
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use crate::rules::{check_direction, enemy, is_legal_move};
use ansi_term::{Colour, Colour::*, Style};
use std::fmt::Display;
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Case {
    Empty,
    White,
    Black,
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

pub struct Board {
    cases: Vec<Vec<Case>>,
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
impl Board {
    /// Create a new board
    /// # Returns
    /// * A new board
    pub fn new() -> Self {
        let mut board = Board {
            cases: vec![vec![Case::Empty; 8]; 8],
        };
        board.cases[3][3] = Case::White;
        board.cases[4][4] = Case::White;
        board.cases[3][4] = Case::Black;
        board.cases[4][3] = Case::Black;
        board
    }
    /// Make a move on the board
    /// # Arguments
    /// * `bmove` - The move to make
    /// * `color` - The color of the player
    /// # Returns
    /// * `Ok(())` if the move is legal
    /// * `Err(())` if the move is illegal
    pub fn make_move(&mut self, bmove: (usize, usize), color: Case) -> Result<(), &str> {
        if !is_legal_move(&self.cases, bmove, &color) {
            return Err("Illegal move");
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
                while self.cases[x as usize][y as usize] == enemy(&color) {
                    self.cases[x as usize][y as usize] = color;
                    x += direction.0;
                    y += direction.1;
                }
            }
        }
        Ok(())
    }
    /// Returns a vector of all the available moves for a given color
    /// # Arguments
    /// * `color` - The color of the player
    pub fn available_moves(&self, color: &Case) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        for i in 0..8 {
            for j in 0..8 {
                if is_legal_move(&self.cases, (i, j), color) {
                    moves.push((i, j));
                }
            }
        }
        moves
    }
    /// Returns the score of the board
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
        let mut string = String::from("   a  b  c  d  e  f  g  h \n");
        let mut bg: u8 = 0;
        for (i, line) in self.cases.iter().enumerate() {
            string.push_str(&format!("{} ", i)[..]);
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
                                .paint(" • ")
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
                                .paint(" • ")
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
fn make_move_test() {
    let mut board = Board::new();
    board
        .make_move((0, 0), Case::White)
        .expect_err("Move should not be legal");
    assert_eq!(board.cases[0][0], Case::Empty); // Check that the move was not made
                                                // Check the initial board
    assert_eq!(board.cases[3][3], Case::White);
    assert_eq!(board.cases[4][4], Case::White);
    assert_eq!(board.cases[3][4], Case::Black);
    assert_eq!(board.cases[4][3], Case::Black);
    println!("{}", board);
}
