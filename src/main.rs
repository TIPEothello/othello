/*
 File: main.rs
 Created Date: 14 Mar 2023
 Author: realbacon
 -----
 Last Modified: 14/03/2023 04:20:14
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/
use ansi_term::{Color, Colour::*, Style};
use std::fmt::Display;
#[derive(Clone)]
enum Case {
    Empty,
    White,
    Black,
}

struct Board {
    cases: Vec<Vec<Case>>,
}

impl Board {
    fn new() -> Self {
        Board {
            cases: vec![vec![Case::Empty; 8]; 8],
        }
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

fn main() {
    let board = Board::new();
    println!("{}", board);
}
