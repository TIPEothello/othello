/*
 File: mcts.rs
 Created Date: 05 Sep 2023
 Author: realbacon
 -----
 Last Modified: 5/09/2023 03:15:2
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

#![allow(non_snake_case, unused_variables, dead_code)]

use core::panic;

use crate::board::{Board, Case};

const EXPLORATION_PARAMETER: f32 = std::f32::consts::SQRT_2;

#[derive(Debug, Clone)]
struct Node {
    played: u32,
    wins: u32,
    action: (usize, usize),
    turn: Case,
    state: Board,
    children: Vec<Node>,
    possible_moves: Vec<(usize, usize)>,
    is_terminal: bool,
}

impl Node {
    fn new_child(&mut self, child: Node) {
        self.children.push(child);
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = String::new();
        res.push_str(&format!("played: {}\n", self.played));
        res.push_str(&format!("wins: {}\n", self.wins));
        res.push_str(&format!("action: {:?}\n", self.action));
        write!(f, "{}", res)
    }
}

impl std::fmt::Display for MCTS {
    // iterate over the nodes recursively and print them
    // use an auxiliary function
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn add_nodes(nodes: &Vec<Node>) -> String {
            let mut s = String::new();
            for node in nodes.iter() {
                s.push_str(&format!("{}\n", node));
                if node.children.len() != 0 {
                    s.push_str(&format!("   {}", add_nodes(&node.children))[..]);
                }
            }
            s
        }
        let mut res = String::new();

        for node in self.root.iter() {
            res.push_str(&format!("{}\n", node));
        }
        write!(f, "{}", res)
    }
}

struct MCTS {
    root: Vec<Node>,
    player: Case,
}

impl MCTS {
    pub fn new(board: Board) -> MCTS {
        let mut init_nodes = Vec::new();
        let mut board = board;
        let player = board.get_turn();
        let moves = board.available_moves(None);
        for mov in moves {
            board.play_move(&mov).unwrap();
            init_nodes.push(Node {
                played: 0,
                wins: 0,
                action: mov,
                turn: board.get_turn().opponent(),
                state: board.clone(),
                children: Vec::with_capacity(8),
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

    fn selection(&self) -> Vec<usize> {
        let mut res = Vec::new();
        fn select_rec(path: &mut Vec<usize>, root: &Vec<Node>, parent: Option<&Node>) {
            let mut nodes = root;

            let parent_played = if let Some(node) = parent {
                node.played
            } else {
                nodes.iter().fold(0, |acc, n| acc + n.played)
            } as f32;

            let idx = choose_UCT(nodes, parent_played);
            path.push(idx);
            let node = &nodes[idx];
            nodes = &node.children;

            if !nodes.is_empty() {
                select_rec(path, nodes, Some(node));
            }
        }
        select_rec(&mut res, &self.root, None);
        res
    }

    fn simulation(node: &mut Node, player: Case) -> Case {
        let mut board = node.state.clone();
        let mut moves = board.available_moves(None);
        let mut win = 0;
        while moves.len() != 0 {
            let idx = rand::random::<usize>() % moves.len();
            board.play_move(&moves[idx]).unwrap();
            moves = board.available_moves(None);
        }
        let winner = {
            let score = board.score();
            if score.0 > score.1 {
                Case::White
            } else if score.0 < score.1 {
                Case::Black
            } else {
                Case::Empty
            }
        };
        winner
    }

    fn backpropagate(root: &mut Vec<Node>, path: Vec<usize>, winner: Case) {
        let mut nodes = root;
        for i in 0..path.len() {
            if nodes[path[i]].turn == winner {
                nodes[path[i]].wins += 1;
            }
            nodes[path[i]].played += 1;
            nodes = &mut nodes[path[i]].children;
        }
    }

    fn extend<'a>(path: &'a Vec<usize>, root: &'a mut Vec<Node>) -> &'a mut Node {
        //println!("{:?}", root);
        let node = root.index_path_mut(path);
        //println!("{:?}", node);
        let board = node.state.clone();
        for i in 0..node.possible_moves.len() {
            let mov = node.possible_moves[i];
            let mut new_state = board.clone();
            new_state.play_move(&mov).unwrap();
            let pm = new_state.available_moves(None);
            let it = pm.len() == 0;
            node.new_child(Node {
                played: 0,
                wins: 0,
                action: mov,
                turn: node.turn.opponent(),
                state: new_state,
                children: Vec::new(),
                possible_moves: pm,
                is_terminal: it,
            });
        }
        node
    }

    fn rebase(self, path: usize) -> Self {
        let root = self.root;
        let rnode = root[path].clone();

        let mut root = rnode.children;
        if root.is_empty() {
            let mut board = rnode.state;
            let moves = rnode.possible_moves;
            for mov in moves {
                board.play_move(&mov).unwrap();
                root.push(Node {
                    played: 0,
                    wins: 0,
                    action: mov,
                    turn: board.get_turn(),
                    state: board.clone(),
                    children: Vec::with_capacity(8),
                    possible_moves: board.available_moves(None),
                    is_terminal: false,
                });
                board.reset(1);
            }
        }
        MCTS {
            root,
            player: self.player,
        }
    }

    fn find_move(&self, mov: (usize, usize)) -> usize {
        for (idx, ch) in self.root.iter().enumerate() {
            if ch.action == mov {
                return idx;
            }
        }
        panic!()
    }
}

trait IndexPath {
    fn index_path_mut(&mut self, path: &Vec<usize>) -> &mut Node;
}

impl IndexPath for Vec<Node> {
    fn index_path_mut(&mut self, path: &Vec<usize>) -> &mut Node {
        let mut nodes = self;
        for i in 0..path.len() - 1 {
            nodes = &mut nodes[path[i]].children;
        }
        &mut nodes[path[path.len() - 1]]
    }
}
fn choose_UCT(nodes: &Vec<Node>, parent_played: f32) -> usize {
    let mut _max = f32::MIN;
    let mut max_index: usize = 0;

    for (i, node) in nodes.iter().enumerate() {
        let val = if node.played > 0 {
            node.wins as f32 / node.played as f32
                + EXPLORATION_PARAMETER
                    * f32::sqrt(f32::ln(parent_played as f32) / node.played as f32)
        } else {
            1.0
        };
        if val > _max {
            _max = val;
            max_index = i;
        }
    }
    max_index
}

fn best_move(nodes: &Vec<Node>) -> usize {
    let mut _max = f32::MIN;
    let mut max_index: usize = 0;

    for (i, node) in nodes.iter().enumerate() {
        let val = if node.played > 0 {
            node.wins as f32 / node.played as f32
        } else {
            0.0
        };
        if val > _max {
            _max = val;
            max_index = i;
        }
    }
    max_index
}

pub(crate) fn test_mcts_mthreads() {
    let mut w = 0;
    use crate::minimax;
    use rayon::prelude::*;
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .unwrap();
    let results: Vec<(usize, usize)> = (0..100)
        .into_par_iter()
        .map(|_| {
            let mut board = Board::new();

            //println!("Started thread ");
            let mut mcts = MCTS::new(board.clone());
            loop {
                let time = std::time::Instant::now();
                loop {
                    //println!("test 1");
                    let selection_path = mcts.selection();
                    //println!("test 2");
                    let mut extended_node = MCTS::extend(&selection_path, &mut mcts.root);
                    //println!("test 3");
                    let w = MCTS::simulation(&mut extended_node, Case::Black);
                    //println!("test 4");
                    MCTS::backpropagate(&mut mcts.root, selection_path, w);
                    //println!("test 5");
                    if time.elapsed().as_millis() >= 400 {
                        break;
                    }
                }
                let best = best_move(&mcts.root);

                let bestn = &mcts.root[best];

                board.play_move(&bestn.action).unwrap();
                if board.available_moves(None).len() == 0 {
                    break;
                }
                mcts = mcts.rebase(best);

                let mut tree = minimax::Tree::from_board(&mut board, None, 4);
                let best_tree = minimax::minimax_tree(&mut tree, board.get_turn());

                let m = board.play_move(&best_tree.mov.unwrap()).unwrap();
                //let m = board.play_move_with_highest_gain().unwrap();
                //let moves = board.available_moves(None);
                //let idx = rand::random::<usize>() % moves.len();
                //let m = board.play_move(&moves[idx]).unwrap();
                if board.available_moves(None).len() == 0 {
                    break;
                }
                let i = mcts.find_move(m);
                mcts = mcts.rebase(i);
            }
            println!("{:?}", board.score());
            board.score()
        })
        .collect();

    for r in results {
        if r.0 <= r.1 {
            w += 1;
        }
    }
    println!("{w}")
}

pub(crate) fn test_mcts() {
    let mut w = 0;
    use crate::minimax;
    let results: Vec<(usize, usize)> = (0..100)
        .into_iter()
        .map(|_| {
            let mut board = Board::new();

            //println!("Started thread ");
            let mut mcts = MCTS::new(board.clone());
            loop {
                let time = std::time::Instant::now();
                loop {
                    let selection_path = mcts.selection();
                    let mut extended_node = MCTS::extend(&selection_path, &mut mcts.root);
                    let w = MCTS::simulation(&mut extended_node, Case::Black);
                    MCTS::backpropagate(&mut mcts.root, selection_path, w);
                    if time.elapsed().as_millis() >= 1000 {
                        break;
                    }
                }
                let best = best_move(&mcts.root);

                let bestn = &mcts.root[best];

                board.play_move(&bestn.action).unwrap();
                if board.available_moves(None).len() == 0 {
                    break;
                }
                mcts = mcts.rebase(best);

                let mut tree = minimax::Tree::from_board(&mut board, None, 4);
                let best_tree = minimax::minimax_tree(&mut tree, board.get_turn());

                let m = board.play_move(&best_tree.mov.unwrap()).unwrap();
                //let m = board.play_move_with_highest_gain().unwrap();
                //let moves = board.available_moves(None);
                //let idx = rand::random::<usize>() % moves.len();
                //let m = board.play_move(&moves[idx]).unwrap();
                if board.available_moves(None).len() == 0 {
                    break;
                }
                let i = mcts.find_move(m);
                mcts = mcts.rebase(i);
            }
            println!("{:?}", board.score());
            board.score()
        })
        .collect();

    for r in results {
        if r.0 <= r.1 {
            w += 1;
        }
    }
    println!("{w}")
}
