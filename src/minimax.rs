/*
 File: strategy.rs
 Created Date: 21 Mar 2023
 Author: realbacon
 -----
 Last Modified: 18/04/2023 12:28:7
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
const FACTOR: f32 = 1.20205690315959428539973816151144999076498629234049888179227155534183820578631309018645587340685;
pub fn minimax(outcomes: &Vec<Vec<(usize, usize)>>, board: &mut Board) -> (usize, usize) {
    let mut best_move = (f32::MIN, (0, 0));
    let mut turn = 1;
    let max_player = board.get_turn();
    for i in 0..outcomes.len() {
        let mut score: f32 = 0.0;

        for j in 0..outcomes[i].len() {
            let ev = evaluate(board, outcomes[i][j], j == outcomes[i].len() - 1) as f32 * FACTOR;
            score = score + ev as f32 * turn as f32;
            turn = turn * -1;
        }
        board.reset(outcomes[i].len());

        if score > best_move.0 as f32 {
            best_move = (score, outcomes[i][0]);
        }
    }
    best_move.1
}
const CORNERS: [(usize, usize); 4] = [(0, 0), (0, 7), (7, 0), (7, 7)];
const AROUND_CORNERS: [(usize, usize); 12] = [
    (0, 1),
    (1, 0),
    (1, 1),
    (0, 6),
    (1, 6),
    (1, 7),
    (6, 0),
    (6, 1),
    (7, 1),
    (6, 6),
    (6, 7),
    (7, 6),
];
pub fn evaluate(board: &mut Board, move_: (usize, usize), last_move: bool) -> i32 {
    let turn = board.get_turn();
    let mut res: isize = 0;
    let score = board.score();

    // Evaluation of the move based on the material count
    let old_material_count = (score.0 - score.1) as isize;
    board.make_move(&move_).unwrap();
    let score = board.score();
    let new_material_count = (score.0 - score.1) as isize;
    if turn == Case::White {
        res = new_material_count - old_material_count;
    } else {
        res = old_material_count - new_material_count;
    }

    // Evalutation of the move based on the corners
    if CORNERS.contains(&move_) {
        res += 15;
    }
    if AROUND_CORNERS.contains(&move_) {
        res -= 10;
    }

    res as i32
}
