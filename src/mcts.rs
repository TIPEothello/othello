/*
 File: mcts.rs
 Created Date: 23 May 2023
 Author: realbacon
 -----
 Last Modified: 23/05/2023 02:20:16
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

// Monte Carlo Tree Search

use std::fmt::Debug;

use crate::board::{self, Board, Case};

// Implement Monte Carlo Tree Search
#[derive(Debug)]
pub struct MCTS {
    tree: Node,
}

#[derive(Debug)]
struct Node {
    wins: i64,
    played: i64,
    leafs: Vec<Node>,
    board: Board,
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn display_rec(
            node: &Node,
            depth: i8,
            f: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            writeln!(f, "{}", node.board.to_string())?;
            for n in node.leafs.iter() {
                display_rec(n, depth + 1, f)?;
            }
            Ok(())
        }
        display_rec(self, 0, f)
    }
}

impl MCTS {
    pub fn new(board: Board) -> Self {
        MCTS {
            tree: Node {
                wins: 0,
                played: 0,
                leafs: Vec::new(),
                board: board,
            },
        }
    }

    pub fn expand_by_depth(&mut self, depth: i8) {
        fn expand_rec(node: &mut Node, depth: i8) {
            if depth == 0 {
                return;
            }
            let moves = node.board.available_moves(None);
            for m in moves {
                let mut board = node.board.clone();
                board.make_move(&m).unwrap();
                node.leafs.push(Node {
                    wins: 0,
                    played: 0,
                    leafs: Vec::new(),
                    board,
                });
            }
            for n in node.leafs.iter_mut() {
                expand_rec(n, depth - 1);
            }
        }
        expand_rec(&mut self.tree, depth)
    }

    fn play_game_from_node(node: &Node) -> Case {
        let mut board = node.board.clone();
        loop {
            let moves = board.available_moves(None);
            if moves.is_empty() {
                break;
            }
            let turn = board.get_turn();
            let mut bestmov: ((usize, usize), isize) = ((0, 0), isize::MIN);
            for mov in moves.iter() {
                let mut board = board.clone();
                board.make_move(mov).unwrap();
                let ev = position_evaluation(&board, turn);
                if ev > bestmov.1 {
                    bestmov = (*mov, ev);
                    break;
                }
            }
            board.make_move(&bestmov.0).unwrap();
        }
        let s = board.score();
        if s.0 > s.1 {
            Case::White
        } else if s.0 < s.1 {
            Case::Black
        } else {
            Case::Empty
        }
    }
    pub fn best_move(&mut self, iter: i32, case: Case) -> (usize, usize) {
        self.play_simulation(iter, case);
        let mut bestmov: ((usize, usize), f32) = ((0, 0), f32::MIN);
        for node in self.tree.leafs.iter() {
            let ev = node.wins as f32 / node.played as f32;
            println!("{}", ev);
            if ev > bestmov.1 {
                bestmov = (*node.board.history.moves.last().unwrap(), ev);
            }
        }
        bestmov.0
    }
    fn play_simulation(&mut self, iter: i32, case: Case) {
        fn rec_play(node: &Node, iter: i32, case: Case) -> (i64, i64) {
            if node.leafs.len() > 0 {
                let mut re = (0, 0);
                for node in node.leafs.iter() {
                    let r = rec_play(node, iter, case);
                    re = (re.0 + r.0, re.1 + r.1);
                }
                return re;
            } else {
                let mut wins = 0;
                let mut played = 0;
                for _ in 0..iter {
                    let mut node = node.clone();
                    let winner = MCTS::play_game_from_node(node);
                    if winner == case {
                        wins += 1;
                    }
                    played += 1;
                }
                return (wins, played);
            }
        }

        for node in self.tree.leafs.iter_mut() {
            let (wins, played) = rec_play(node, iter, case);
            node.wins += wins;
            node.played += played;
        }
    }
}

#[test]
fn create_mcts() {
    let mut mcts = MCTS::new(Board::new());
    mcts.expand_by_depth(3);
    let bmov = mcts.best_move(50, Case::Black);
    println!("{:?}", bmov);
}

use crate::minimax::matrix_eval;
fn position_evaluation(board: &Board, case: Case) -> isize {
    let ev = matrix_eval(&board.cases);
    match case {
        Case::White => ev.0,
        Case::Black => ev.1,
        _ => unreachable!(),
    }
}
