/*
 File: strategy.rs
 Created Date: 21 Mar 2023
 Author: realbacon
 -----
 Last Modified: 19/04/2023 05:04:57
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
use rayon::prelude::*;
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
        let mut res = String::new();
        fn rec(tree: &Tree, depth: u8, res: &mut String) {
            for _ in 0..depth {
                res.push_str("   ");
            }
            res.push_str(&format!(
                "Depth: {}, Moves: {}, Score: {:?}, Value: {:?}, Move: {:?}",
                tree.depth, tree.moves, tree.score, tree.value, tree.mov
            ));
            res.push_str("\n");
            if let Some(subtree) = &tree.subtree {
                for sub in subtree {
                    rec(sub, depth + 1, res);
                }
            }
        }
        rec(self, 0, &mut res);
        write!(f, "{}", res)
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
            let ev = evaluate(board, outcomes[i][j]) as f32;
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
    [128, -5, 30, 10, 10, 30, -5, 128],
    [-5, -8, 0, 0, 0, 0, -8, -5],
    [30, 0, 1, 2, 2, 1, 0, 30],
    [10, 0, 2, 0, 0, 2, 0, 10],
    [10, 0, 2, 0, 0, 2, 0, 10],
    [30, 0, 1, 2, 2, 1, 0, 30],
    [-5, -8, 0, 0, 0, 0, -8, -5],
    [128, -5, 30, 10, 10, 30, -5, 128],
];
pub fn evaluate(board: &mut Board, move_: (usize, usize)) -> i32 {
    let turn = board.get_turn();
    let res: isize;
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
    //res -= board.available_moves(None).len() as isize * 5;
    //res += (PLACEMENT_SCORE[move_.0][move_.1] as f32) as isize;

    res as i32
}

pub fn minimax_tree(tree: &mut Tree, color: Case) -> Tree {
    pub fn minimax_rec(
        original_score: (usize, usize),
        tree: &mut Tree,
        color: Case,
        current: Case,
        mut alpha: i32,
        mut beta: i32,
    ) -> i32 {
        if tree.moves == 0 || tree.subtree.is_none() {
            let val = evaluate_tree(original_score, &tree, color, tree.mov.unwrap(), current);
            tree.value = Some(val);
            return val;
        }
        let mut best = if current == color { i32::MIN } else { i32::MAX };
        for subtree in tree.subtree.as_mut().unwrap() {
            let val = minimax_rec(original_score, subtree, color, enemy(&current), alpha, beta);
            if color == current {
                best = best.max(val);
                alpha = alpha.max(val);
                if best >= beta {
                    break;
                }
            } else {
                best = best.min(val);
                beta = beta.min(val);
                if best <= alpha {
                    break;
                }
            }
        }
        tree.value = Some(best);
        return best;
    }

    let best = minimax_rec(tree.score, tree, color, color, i32::MIN, i32::MAX);
    let best_tree = tree
        .subtree
        .as_ref()
        .unwrap()
        .par_iter()
        .find_any(|x| x.value.unwrap() == best)
        .unwrap()
        .clone();

    return best_tree;
}

pub fn matrix_eval(cases: &[[Case; 8]; 8]) -> (isize, isize) {
    let mut res = (0, 0);
    for i in 0..8 {
        for j in 0..8 {
            if cases[i][j] == Case::White {
                res.0 += PLACEMENT_SCORE[i][j];
            } else if cases[i][j] == Case::Black {
                res.1 += PLACEMENT_SCORE[i][j];
            }
        }
    }
    res
}

pub fn evaluate_tree(
    original_score: (usize, usize),
    tree: &Tree,
    color: Case,
    move_next: (usize, usize),
    turn: Case,
) -> i32 {
    let res: isize = 15 * material_count(original_score, tree, color)
        + position_evluation(tree, color)
        + 3 * randomness_factor();

    res as i32
}

fn material_count(original_score: (usize, usize), tree: &Tree, color: Case) -> isize {
    let mut res = (tree.score.0 - tree.score.1 - original_score.0 + original_score.1) as isize;
    if color == Case::Black {
        res = res * -1;
    }
    res
}

fn position_evluation(tree: &Tree, color: Case) -> isize {
    if color == Case::White {
        matrix_eval(&tree.cases).0 as isize
    } else {
        matrix_eval(&tree.cases).1 as isize
    }
}

fn randomness_factor() -> isize {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let random: isize = rng.gen_range(-10..10);
    random
}
