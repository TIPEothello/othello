/*
 File: mcts.rs
 Created Date: 13 Jun 2023
 Author: realbacon
 -----
 Last Modified: 27/06/2023 02:18:39
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/
#![allow(non_snake_case, unused_variables, dead_code)]

use crate::board::{Board, Case};

const EXPLORATION_PARAMETER: f32 = 1.41421356;

#[derive(Debug)]
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

const SIMULATION_ITERATION: usize = 50;

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
                turn: board.get_turn(),
                state: board.clone(),
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
                println!("{:?}", nodes[path[i]]);
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

    fn simulation(node: &mut Node, player: Case) -> (u32, u32) {
        let mut win = 0;
        let mut played = 0;
        for child in node.children.iter_mut() {
            if !child.is_terminal {
                for _ in 0..SIMULATION_ITERATION {
                    let mut board = child.state.clone();
                    let mut moves = board.available_moves(None);
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

                    if winner == player.opponent() && child.turn == player.opponent() {
                        child.wins += 1;
                        win += 1
                    } else if winner == player && child.turn == player {
                        child.wins += 1;
                        win += 1
                    }
                    child.played += 1;
                    played += 1;
                }
            }
        }
        (win, played)
    }

    fn backpropagate(root: &mut Vec<Node>, path: Vec<usize>, wins: u32, played: u32) {
        let mut nodes = root;
        for i in 0..path.len() {
            nodes[path[i]].wins += wins;
            nodes[path[i]].played += played;
            nodes = &mut nodes[path[i]].children;
        }
    }

    fn extend<'a>(path: &'a Vec<usize>, root: &'a mut Vec<Node>) -> &'a mut Node {
        //println!("{:?}", root);
        let node = root.index_path(path);
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
}

trait IndexPath {
    fn index_path(&mut self, path: &Vec<usize>) -> &mut Node;
}

impl IndexPath for Vec<Node> {
    fn index_path(&mut self, path: &Vec<usize>) -> &mut Node {
        let mut nodes = self;
        for i in 0..path.len() - 1 {
            nodes = &mut nodes[path[i]].children;
        }
        &mut nodes[path[path.len() - 1]]
    }
}
fn choose_UCT(nodes: &Vec<Node>) -> usize {
    let mut _max = f32::MIN;
    let mut max_index: usize = 0;

    for (i, node) in nodes.iter().enumerate() {
        let val = if node.played > 0 {
            node.wins as f32 / node.played as f32
                + EXPLORATION_PARAMETER
                    * f32::sqrt(f32::ln(node.played as f32) / node.played as f32)
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
#[test]
fn test_mcts() {
    let board = Board::new();
    let mut mcts = MCTS::new(board.clone());
    for _ in 0..10 {
        let selection_path = MCTS::selection(&mcts.root);
        let mut extended_node = MCTS::extend(&selection_path, &mut mcts.root);
        let (w, p) = MCTS::simulation(&mut extended_node, Case::Black);
        MCTS::backpropagate(&mut mcts.root, selection_path, w, p);
    }
    println!("{}", mcts);
}
