#![allow(dead_code, unused_variables)]

use crate::board::{Board, Case};
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
        if depth == 0 || moves.is_empty() {
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
                board.play_move(mov).unwrap();
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

    pub fn best_move(&mut self, color: Case) -> (usize, usize) {
        minimax(self, color).mov.unwrap()
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
            res.push('\n');
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

pub const PLACEMENT_SCORE: [[isize; 8]; 8] = [
    [256, -8, 16, 16, 16, 16, -8, 256],
    [-8, -8, -4, -4, -4, -4, -8, -8],
    [16, -4, 0, 0, 0, 0, -4, 16],
    [16, -4, 0, 0, 0, 0, -4, 16],
    [16, -4, 0, 0, 0, 0, -4, 16],
    [16, -4, 0, 0, 0, 0, -4, 16],
    [-8, -8, -4, -4, -4, -4, -8, -8],
    [256, -8, 16, 16, 16, 16, -8, 256],
];

pub fn minimax(tree: &mut Tree, color: Case) -> Tree {
    pub fn minimax_rec(
        original_score: (usize, usize),
        tree: &mut Tree,
        color: Case,
        current: Case,
        mut alpha: i32,
        mut beta: i32,
    ) -> i32 {
        if tree.moves == 0 || tree.subtree.is_none() {
            let val = evaluate(original_score, tree, color);
            tree.value = Some(val);
            return val;
        }
        let mut best = if current == color { i32::MIN } else { i32::MAX };
        for subtree in tree.subtree.as_mut().unwrap() {
            let val = minimax_rec(
                original_score,
                subtree,
                color,
                current.opponent(),
                alpha,
                beta,
            );
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
        best
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

    best_tree
}
/// (white,black)
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

pub fn evaluate(original_score: (usize, usize), tree: &Tree, color: Case) -> i32 {
    let score = tree.score;
    let filled = score.0 + score.1;
    let balance = if color == Case::Black {
        score.0 as i32 - score.1 as i32
    } else {
        score.1 as i32 - score.0 as i32
    };
    if tree.moves == 0 {
        10000 * balance.signum() + balance
    } else {
        let state = filled / 16; // Découpe la partie en plusieurs phases
        let mut result = balance << state; // On attribue une importance grandissante au score en fonction de l'avancement de la partie
        let matrix = matrix_eval(&tree.cases);
        result += if color == Case::Black {
            matrix.1 as i32 - matrix.0 as i32 // Différence de score de placement
        } else {
            matrix.0 as i32 - matrix.1 as i32
        };
        result
    }
}

fn material_count(original_score: (usize, usize), tree: &Tree, color: Case) -> isize {
    let mut res = (tree.score.0 - tree.score.1 - original_score.0 + original_score.1) as isize;
    if color == Case::White {
        res = -res;
    }
    res
}

fn position_evaluation(tree: &Tree, color: Case) -> isize {
    if color == Case::White {
        matrix_eval(&tree.cases).0
    } else {
        matrix_eval(&tree.cases).1
    }
}

fn randomness_factor() -> isize {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let random: isize = rng.gen_range(-10..10);
    random
}

pub fn freedom_factor(tree: &Tree) -> isize {
    tree.moves as isize
}
