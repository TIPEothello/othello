/*
 File: mcts.rs
 Created Date: 13 Jun 2023
 Author: realbacon
 -----
 Last Modified: 20/06/2023 02:24:30
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/
#[allow(non_snake_case)]
use crate::board::{Board, Case};

const EXPLORATION_PARAMETER: f32 = 1.41421356;

#[derive(Debug)]
struct Node {
    played: u32,
    wins: u32,
    action: (usize, usize),
    turn: Case,
    state: (u64, u64),
    children: Vec<Node>,
    possible_moves: Vec<(usize, usize)>,
    is_terminal: bool,
}
struct MCTS {
    root: Vec<Node>,
    player: Case,
}

impl MCTS {
    pub fn new(board: (u64, u64)) -> MCTS {
        let mut init_nodes = Vec::new();
        let mut board = Board::from_u64(board);
        let player = board.get_turn();
        let moves = board.available_moves(None);
        for mov in moves {
            board.make_move(&mov).unwrap();
            init_nodes.push(Node {
                played: 0,
                wins: 0,
                action: mov,
                turn: board.get_turn(),
                state: board.to_u64(),
                children: Vec::new(),
                possible_moves: board.available_moves(None),
                is_terminal: false,
            });
            board.reset(1);
        }
        MCTS {
            root: init_nodes,
            player,
        }
    }

    fn selection(root: &Vec<Node>) -> Vec<usize> {
        let mut res = Vec::new();
        fn select_rec(path: &mut Vec<usize>, root: &Vec<Node>) {
            let mut nodes = root;
            for i in 0..path.len() {
                nodes = &nodes[path[i]].children;
            }
            let idx = choose_UCT(nodes);
            path.push(idx);
            if nodes[idx].children.len() != 0 {
                select_rec(path, nodes);
            }
        }
        select_rec(&mut res, root);
        res
    }

    fn explore(path: &mut Vec<usize>, root: &mut Vec<Node>) {
        let mut root = root[path[0]];

        for i in 0..path.len() {
            let a = root.last().unwrap();
            root = &mut root[path[i]];
        }
    }
}

fn choose_UCT(nodes: &Vec<Node>) -> usize {
    let mut _max = f32::MIN;
    let mut max_index: usize = 0;

    for (i, node) in nodes.iter().enumerate() {
        println!("{:?}", node);
        let val = node.wins as f32 / node.played as f32
            + EXPLORATION_PARAMETER * f32::sqrt(f32::ln(node.played as f32) / node.played as f32);
        println!("{}", val);
        if val > _max {
            _max = val;
            max_index = i;
        }
    }
    max_index
}
#[test]
fn test_mcts() {
    let mut board = Board::new();
    let mcts = MCTS::new(board.to_u64());
    println!("{:?}", mcts.root);
    let selection_path = MCTS::selection(&mcts.root);
    println!("{:?}", selection_path)
}
