/*
 File: board.rs
 Created Date: 14 Mar 2023
 Author: realbacon
 -----
 Last Modified: 14/03/2023 05:11:39
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use crate::rules::{check_direction, enemy, is_legal_move};
use ansi_term::{Colour::*, Style};
use std::fmt::Display;
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Case {
    Empty,
    White,
    Black,
}

pub struct Board {
    cases: Vec<Vec<Case>>,
}

impl Board {
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

    pub fn make_move(&mut self, bmove: (usize, usize), color: Case) -> Result<(), ()> {
        if !is_legal_move(&self.cases, bmove, &color) {
            return Err(());
        }
        self.cases[bmove.0][bmove.1] = color;

        for direction in vec![
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ] {
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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut string = String::new();
        for line in self.cases.iter() {
            for case in line.iter() {
                match case {
                    Case::Empty => {
                        string.push_str(
                            Style::new()
                                .on(RGB(150, 150, 150))
                                .paint("ㅤ")
                                .to_string()
                                .as_str(),
                        );
                    }
                    Case::White => {
                        string.push_str(Style::new().on(White).paint("ㅤ").to_string().as_str());
                    }
                    Case::Black => {
                        string.push_str(Style::new().on(Black).paint("ㅤ").to_string().as_str());
                    }
                }
            }
            string += "\n";
        }
        write!(f, "{}", string)
    }
}

#[test]
fn make_move_test() {
    let mut board = Board::new();
    board.make_move((0, 0), Case::White).unwrap();
    assert_eq!(board.cases[0][0], Case::White);
    assert_eq!(board.cases[3][3], Case::White);
    assert_eq!(board.cases[4][4], Case::White);
    assert_eq!(board.cases[3][4], Case::Black);
    assert_eq!(board.cases[4][3], Case::Black);
    println!("{}", board);
}
