/*
 File: mcts.rs
 Created Date: 23 May 2023
 Author: realbacon
 -----
 Last Modified: 30/05/2023 02:24:13
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

// Monte Carlo Tree Search

use std::fmt::Debug;

use crate::board::{Board, Case};
use crate::minimax::PLACEMENT_SCORE;
// Implement Monte Carlo Tree Search
#[derive(Debug)]
pub struct MCTS {
    tree: Vec<Node>,
    board: Board,
    color: Case,
}

#[derive(Debug, Clone)]
struct Node {
    parent: Option<Box<Node>>,
    wins: i64,
    played: i64,
    leafs: Vec<Node>,
    legal_moves: Vec<(usize, usize)>,
    parent_move: Option<(usize, usize)>,
    depth: usize,
    untried_action: Vec<(usize, usize)>,
}

impl MCTS {
    pub fn new(board: Board) -> Self {
        let color = board.get_turn();
        MCTS {
            tree: Vec::new(),
            board,
            color,
        }
    }

    fn play_to_node(&mut self, node: &Node) {
        if let Some(p) = node.parent.as_ref() {
            self.play_to_node(p.as_ref());
            let m = p.parent_move.unwrap();
            self.board.make_move(&m).unwrap();
        }
    }

    fn expand_node(&mut self, node: &mut Node) {
        self.play_to_node(node);
        for mov in self.board.available_moves(None).iter() {
            self.board.make_move(mov).unwrap();
            let m = self.board.available_moves(None);
            node.leafs.push(Node {
                wins: 0,
                played: 0,
                leafs: Vec::new(),
                legal_moves: m.clone(),
                untried_action: m,
                parent: Some(Box::new(node.clone())),
                parent_move: Some(*mov),
                depth: node.depth + 1,
            });
            self.board.reset(1);
        }
        self.board.reset(node.depth - 1);
    }

    pub fn init_first_depth(&mut self) {
        let legal_actions = self.board.available_moves(None);
        for mov in legal_actions.iter() {
            self.board.make_move(mov).unwrap();
            let m = self.board.available_moves(None);
            self.tree.push(Node {
                wins: 0,
                played: 0,
                leafs: Vec::new(),
                legal_moves: m.clone(),
                untried_action: m,
                parent: None,
                parent_move: Some(*mov),
                depth: 1,
            });
            self.board.reset(1);
        }
    }

    fn select_in_layer(&mut self, layer: Vec<Node>) -> Vec<Node> {
        let mut best = Vec::new();
        let mut best_score = 0.0;
        for node in layer {
            if node.is_terminal() {
                best.push(node);
                continue;
            }
            let score = node.wins as f64 / node.played as f64
                + (PLACEMENT_SCORE[node.parent_move.unwrap().0][node.parent_move.unwrap().1]
                    as f64
                    / 100.0);
            if score > best_score {
                best_score = score;
                best = vec![node.clone()];
            } else if score == best_score {
                best.push(node.clone());
            }
        }
        best
    }
}

impl Node {
    pub fn is_terminal(&self) -> bool {
        self.legal_moves.is_empty()
    }

    pub fn backpropagate(&mut self, result: i8) {
        self.played += 1;
        if self.parent.is_some() {
            self.parent.as_mut().unwrap().backpropagate(result);
        }
    }
}

#[test]
fn create_mcts() {
    let mut mcts = MCTS::new(Board::new());
    mcts.init_first_depth();
}

use crate::minimax::matrix_eval;
fn position_evaluation(board: &Board, case: Case) -> isize {
    let ev = matrix_eval(&board.cases);
    match case {
        Case::White => ev.0,
        Case::Black => ev.1,
        _ => unreachable!(),
    }
    /*
    match case {
        Case::White => board.score().0 as isize - board.score().1 as isize,
        Case::Black => board.score().1 as isize - board.score().0 as isize,
        _ => unreachable!(),
    }*/
}
