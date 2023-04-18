/*
 File: strategy.rs
 Created Date: 21 Mar 2023
 Author: realbacon
 -----
 Last Modified: 18/04/2023 01:40:39
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/
#![allow(dead_code)]

use crate::{
    board::{Board, Case},
    rules::enemy,
};
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct Tree {
    pub depth: u8,
    pub subtree: Option<Vec<Tree>>,
	pub moves: usize,
    pub mov: Option<(usize, usize)>,
	pub score: (usize, usize),
    pub cases: [[Case; 8]; 8],
    pub value: Option<i32>,
}

impl Tree {
    pub fn from_board(board: &mut Board, mov: Option<(usize, usize)>, depth: u8) -> Self {
        let moves = board.available_moves(None);
        if depth == 0 || moves.len() == 0 {
            Tree {
                depth: 0,
                subtree: None,
                moves: moves.len(),
                cases: board.cases,
				score: board.score(),
                value: None,
                mov,
            }
        } else {
            let mut subtrees = Vec::new();
            for mov in &moves {
                board.make_move(&mov).unwrap();
                subtrees.push(Tree::from_board(board, Some(*mov), depth - 1));
                board.reset(1);
            }
            Tree {
                depth,
                subtree: Some(subtrees),
				moves: moves.len(),
				score: board.score(),
                cases: board.cases,
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
pub fn calculate_outcomes(board: &Board, depth: u8) -> Vec<Vec<(usize, usize)>> {
    let mut board = board.clone();
    let outcomes = vec![vec![]];
    // calculate all the possible outcomes for the given depth

    fn calc_rec(
        board: &mut Board,
        depth: u8,
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
    let mut best_move = (f32::MIN, (0, 0));
    let mut turn = 1;
    for i in 0..outcomes.len() {
        let mut score: f32 = 0.0;

        for j in 0..outcomes[i].len() {
            let ev = evaluate(board, outcomes[i][j], j == outcomes[i].len() - 1) as f32;
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
const PLACEMENT_SCORE: [[isize; 8]; 8] = [
    [30, -25, 10, 5, 5, 10, -25, 30],
    [-25, -25, 1, 1, 1, 1, -25, -25],
    [10, 1, 5, 2, 2, 5, 1, 10],
    [5, 1, 2, 1, 1, 2, 1, 5],
    [5, 1, 2, 1, 1, 2, 1, 5],
    [10, 1, 5, 2, 2, 5, 1, 10],
    [-25, -25, 1, 1, 1, 1, -25, -25],
    [30, -25, 10, 5, 5, 10, -25, 30],
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
        res = ((new_material_count - old_material_count) * 3) as isize;
    } else {
        res = old_material_count - new_material_count;
    }
    res -= board.available_moves(None).len() as isize * 5;
    res += (PLACEMENT_SCORE[move_.0][move_.1] as f32) as isize;

    res as i32
}


pub fn minimax_tree(tree: &mut Tree, color: Case) -> Tree {
    pub fn minimax_rec(original_score: (usize, usize), tree: &mut Tree, color: Case, current: Case) -> i32 {
        if tree.moves == 0 || tree.subtree.is_none() {
            let val = evaluate_tree(original_score, &tree, current, tree.mov.unwrap());
            tree.value = Some(val);
            return val;
        }
        let mut best = i32::MIN;
        for subtree in tree.subtree.as_mut().unwrap() {
            let val = minimax_rec(original_score, subtree, color, enemy(&current));
            if color == current {
                if val > best {
                    best = val;
                }
            } else {
                if val < best {
                    best = val;
                }
            }
        }
        tree.value = Some(best);
        return best;
    }

    let best = minimax_rec(tree.score, tree, color, color);
    let best_tree = tree.subtree.as_ref().unwrap().iter().find(|x| x.value.unwrap() == best).unwrap().clone();
    return best_tree;
}



pub fn evaluate_tree(original_score: (usize, usize), tree: &Tree, turn: Case, move_next: (usize, usize)) -> i32 {
    let mut res: isize = 0;

    // Evaluation of the move based on the material count
    let old_material_count = (original_score.0 - original_score.1) as isize;

    let score = tree.score;
    let new_material_count = (score.0 - score.1) as isize;
    if turn == Case::White {
        res = ((new_material_count - old_material_count) * 3) as isize;
    } else {
        res = old_material_count - new_material_count;
    }
    res -= tree.moves as isize * 5;
    res += (PLACEMENT_SCORE[move_next.0][move_next.1] as f32) as isize;

    res as i32
}