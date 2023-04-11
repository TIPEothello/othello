/*
 File: strategy.rs
 Created Date: 21 Mar 2023
 Author: realbacon
 -----
 Last Modified: 4/04/2023 02:16:42
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/
#![allow(dead_code)]

use crate::board::{Board, Case};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct Tree {
    pub depth: u8,
    pub subtree: Option<Vec<Tree>>,
    pub mov: Option<(usize, usize)>,
    pub value: Option<i8>,
}

impl Tree {
    pub fn from_board(board: &mut Board, mov: Option<(usize, usize)>, depth: u8) -> Self {
        let moves = board.available_moves(None);
        if depth == 0 || moves.len() == 0 {
            Tree {
                depth: 0,
                subtree: None,
                value: None,
                mov,
            }
        } else {
            let mut subtrees = Vec::new();
            for mov in moves {
                board.make_move(&mov).unwrap();
                subtrees.push(Tree::from_board(board, Some(mov), depth - 1));
                board.reset(1);
            }
            Tree {
                depth,
                subtree: Some(subtrees),
                value: None,
                mov,
            }
        }
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tree<Depth: {}>", self.depth)
    }
}



/// A function  that takes a board and a turn and returns a vector of all possible outcomes
/// # Arguments
/// * `board` - The board to calculate the outcomes from
/// * `turn` - The turn to calculate the outcomes for
///
pub fn calculate_outcomes(board: &Board, depth: i8) -> Vec<Vec<(usize, usize)>> {
    let mut board = board.clone();
    let outcomes = vec![vec![]];
    // calculate all the possible outcomes for the given depth

    fn calc_rec(
        board: &mut Board,
        depth: i8,
        outcomes: Vec<Vec<(usize, usize)>>,
    ) -> Vec<Vec<(usize, usize)>> {
        let mut outcomes = outcomes;
        if depth == 0 {
            //println!("test {:?}", outcomes);
            return outcomes;
        }

        let mut new_outcomes = Vec::new();
        for _ in 0..outcomes.len() {
            //println!("test1 {:?}", outcomes);
            let outcome = outcomes.pop().unwrap_or(vec![]);

            //println!("test2 {:?}", outcome);
            board.play_moves(&outcome).expect("Played illegal moves");
            for (x, y) in board.available_moves(Some(board.get_turn())) {
                let mut new_outcome = outcome.clone();
                new_outcome.push((x, y));
                new_outcomes.push(new_outcome);
            }
            //println!("{:?}", new_outcomes);
            board.reset(outcome.len());
        }
        let new_outcomes = calc_rec(board, depth - 1, new_outcomes);
        new_outcomes
    }
    let outcomes = calc_rec(&mut board, depth, outcomes);

    outcomes
}

pub fn minimax(outcomes: &Vec<Vec<(usize, usize)>>, board: &mut Board) -> (usize, usize) {
    let mut best_move = (isize::MIN, (0, 0));
    let mut turn = -1;
    for i in 0..outcomes.len() {
        let mut score: isize = 0;

        for j in 0..outcomes[i].len() {
            board.make_move(&outcomes[i][j]).unwrap();

            let ev = evalutate(&board, board.get_turn()) as isize;
            score = score + ev * turn;
            turn = turn * -1;
        }
        board.reset(outcomes[i].len());

        if score > best_move.0 {
            best_move = (score, outcomes[i][0]);
        }
    }
    best_move.1
}

pub fn evalutate(board: &Board, turn: Case) -> i32 {
    let mut res = 0;
    for i in 0..8 {
        for j in 0..8 {
            match board.cases[i][j] {
                Case::Empty => {}
                Case::White => {
                    if turn == Case::White {
                        res += 1
                    } else {
                        res -= 1
                    }
                }
                Case::Black => {
                    if turn == Case::Black {
                        res += 1
                    } else {
                        res -= 1
                    }
                }
            }
        }
    }
    res
}
