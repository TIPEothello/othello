use crate::board::{Board, Case};
use rayon::prelude::*;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct Tree {
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
                subtree: Some(subtrees),
                moves: moves.len(),
                score: board.score(),
                cases: board.cases,
                value: None,
                mov,
            }
        }
    }

    pub fn expand_tree(&mut self, mut board: &mut Board, depth: u8) {
        if depth == 0 {
            return;
        }
        if self.subtree.is_none() {
            let mut subtree = Vec::new();
            for m in board.available_moves(None) {
                board.play_move(&m).unwrap();
                subtree.push(Tree::from_board(&mut board, Some(m), depth - 1));
                board.reset(1);
            }
            self.subtree = Some(subtree);
        } else {
            for subt in self.subtree.as_mut().unwrap() {
                board.play_move(&subt.mov.unwrap()).unwrap();
                subt.expand_tree(&mut board, depth - 1);
                board.reset(1);
            }
        }
    }

    pub fn best_move(&mut self, color: Case, board: &Board, depth: u8) -> (usize, usize) {
        self.expand_tree(&mut board.clone(), depth);
        let m = minimax(self, color).mov.unwrap();
        // on coupe l'arbre
        for i in 0..self.moves {
            if self.subtree.as_ref().unwrap()[i].mov.unwrap() == m {
                let subt = self.subtree.as_ref().unwrap()[i].clone();
                self.subtree = subt.subtree;
                self.moves = subt.moves;
                self.cases = subt.cases;
                self.score = subt.score;
                self.value = subt.value;
                self.mov = subt.mov;
                return m;
            }
        }
        panic!("No move found");
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
                " Moves: {}, Score: {:?}, Value: {:?}, Move: {:?}",
                tree.moves, tree.score, tree.value, tree.mov
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
        tree: &mut Tree,
        max_color: Case,
        current_color: Case,
        mut alpha: i32,
        mut beta: i32,
    ) -> i32 {
        if tree.moves == 0 || tree.subtree.is_none() {
            let val = evaluate(tree, max_color);
            tree.value = Some(val);
            return val;
        }
        let mut best = if current_color == max_color {
            i32::MIN
        } else {
            i32::MAX
        };
        for subtree in tree.subtree.as_mut().unwrap() {
            let val = minimax_rec(subtree, max_color, current_color.opponent(), alpha, beta);
            if max_color == current_color {
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

    let best = minimax_rec(tree, color, color, i32::MIN, i32::MAX);
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

pub fn evaluate(tree: &Tree, color: Case) -> i32 {
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
