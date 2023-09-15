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
use std::collections::HashMap;

use rand::{seq::SliceRandom, thread_rng};

use crate::board::{Board, BoardState, Case, EndState};

const EXPLORATION_PARAMETER: f64 = std::f64::consts::SQRT_2;

#[derive(Debug, Clone)]
struct Node {
    played: u32,
    wins: u32,
    turn: Case,
    state: Board,
    children: HashMap<(usize, usize), Node>,
    is_terminal: bool,
    is_fully_expanded: bool,
    exploration_constant: f64,
}

impl Node {
    fn from_expansion(parent: &Node, move_: (usize, usize)) -> (Node, EndState) {
        let mut board = parent.state.clone();
        let player = parent.turn.opponent();
        let (is_end_state, endstate) = match board.play_move(&move_) {
            Ok(BoardState::Ongoing) => (
                false,
                Node::simulate_random_playout(&mut board.clone(), player.opponent()),
            ),
            Ok(BoardState::Ended(endstate)) => (true, endstate),
            Err(msg) => panic!(
                "error in Node.from_expansion when calling board.play_move(): {}",
                msg
            ),
        };
        let mut node = Node {
            state: board,
            turn: player,
            is_terminal: is_end_state,
            is_fully_expanded: is_end_state,
            played: 0,
            wins: 0,
            children: HashMap::new(),
            exploration_constant: parent.exploration_constant,
        };

        node.update_from_endstate(endstate);

        (node, endstate)
    }

    fn simulate_random_playout(board: &mut Board, player: Case) -> EndState {
        let mut curr_player = player.opponent();
        loop {
            let mut rng = thread_rng();
            let game_state =
                board.play_move(board.available_moves(None).choose(&mut rng).unwrap());
            curr_player = curr_player.opponent();
            match game_state {
                Ok(state) => match state {
                    BoardState::Ongoing => (),
                    BoardState::Ended(endstate) => return endstate,
                },
                Err(msg) => panic!("Err in Node.simulate_random_playout(): {}", msg),
            }
        }
    }

    fn update_from_endstate(&mut self, endstate: EndState) {
        self.played += 1;
        let EndState::Winner(winner) = endstate;
        if winner == self.turn {
            self.wins += 1
        }
        
    }

    fn update_fully_expanded(&mut self) {
        if self.children.len() == self.state.available_moves(None).len()
            && self
                .children
                .values()
                .filter(|&c| !c.is_fully_expanded)
                .count()
                == 0
        {
            self.is_fully_expanded = true;
        }
    }

    fn get_exploration_score(&self, exploration_constant: f64, move_: &(usize, usize)) -> f64 {
        // Ratio of simulations from the given node that we didn't lose
        let get_node_not_loss_ratio = |n: &Node| n.wins as f64 / n.played as f64;
        // UCT formula
        let calculate_uct = |score: f64, c_visits: u32, p_visits: u32| {
            score + exploration_constant * (f64::ln(p_visits as f64) / c_visits as f64)
        };
        match self.children.get(move_) {
            Some(child) => {
                if child.is_fully_expanded {
                    0.
                } else {
                    calculate_uct(get_node_not_loss_ratio(child), child.played, self.played)
                }
            }
            None => exploration_constant,
        }
    }

    fn expand(&mut self) -> EndState {
        if self.is_fully_expanded {
            return EndState::Winner(Case::Empty);
        }

        let mut moves = self.state.available_moves(None);
        for (index, move_) in moves.iter().enumerate() {
            if self.children.len() <= index {
                let (child_node, endstate) = Node::from_expansion(self, *move_);
                self.children.insert(*move_, child_node);
                self.update_from_endstate(endstate);
                self.update_fully_expanded();
                return endstate;
            }
        }

        moves.sort_by(|m1, m2| {
            self.get_exploration_score(self.exploration_constant, m1)
                .partial_cmp(&self.get_exploration_score(self.exploration_constant, m2))
                .unwrap()
        });

        while !moves.is_empty() {
            let best_move = moves.pop().unwrap();
            let child = self.children.get_mut(&best_move).unwrap();
            if !child.is_fully_expanded {
                let endstate = child.expand();
                self.update_from_endstate(endstate);
                self.update_fully_expanded();
                return endstate;
            }
        }

        panic!("Expand is broken !")
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = String::new();
        res.push_str(&format!("played: {}\n", self.played));
        res.push_str(&format!("wins: {}\n", self.wins));
        write!(f, "{}", res)
    }
}

impl std::fmt::Display for MCTS {
    // iterate over the nodes recursively and print them
    // use an auxiliary function
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn add_nodes(node: &Node, limit: usize) -> String {
            if limit == 0 {
                return "".to_string();
            }
            let mut s = String::new();
            for node in node.children.values() {
                s.push_str(&format!("{}\n", node));
                if !node.children.is_empty() {
                    s.push_str(&format!("   {}", add_nodes(node, limit - 1))[..]);
                }
            }
            s
        }
        let mut res = String::new();

        res.push_str(&format!("{}\n", add_nodes(&self.root, 5)));
        write!(f, "{}", res)
    }
}

#[derive(Debug, Clone)]
pub struct MCTS {
    root: Node,
    player: Case,
    playout_budget: usize,
    exploration_constant: f64,
}

impl MCTS {
    pub fn new(player: Case, playout_budget: usize, board: Board) -> MCTS {
        let root = Node {
            state: board,
            turn: player.opponent(),
            is_fully_expanded: false,
            is_terminal: false,
            played: 0,
            wins: 0,
            children: HashMap::new(),
            exploration_constant: EXPLORATION_PARAMETER,
        };

        MCTS {
            player,
            playout_budget,
            root,
            exploration_constant: EXPLORATION_PARAMETER,
        }
    }

    pub fn search(&mut self, board: &Board) -> (usize, usize) {
        let opp_move = self.get_opponents_last_move(board);
        if let Some(opp_move_fr) = opp_move {
            self.update_with_opponents_move(opp_move_fr, board)
        }

        let num_moves = board.available_moves(None).len();
        while self.root.children.len() < num_moves {
            self.root.expand();
        }

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(8)
            .build()
            .unwrap();

		pool.scope(|scoped| {
			let playout_budget = self.playout_budget;
            for child in self.root.children.values_mut() {
                scoped.spawn(move |_| {
                    for _ in 0..playout_budget / num_moves {
                        child.expand();
                    }
                });
            }
		});

		self.get_best_move_and_promote_child()
    }

    fn get_opponents_last_move(&self, board: &Board) -> Option<(usize, usize)> {
        board.history.moves.last().copied()
    }

    fn update_with_opponents_move(&mut self, opp_move: (usize, usize), board: &Board) {
        match self.root.children.remove(&opp_move) {
            Some(child_node) => self.root = child_node,
            None => self.root.state = board.clone(),
        }
    }

    fn get_best_move_and_promote_child(&mut self) -> (usize, usize) {
        let mut best_winrate = 0.0f64;
        let mut best_moves: Vec<(usize, usize)> = vec![];

        for (&move_, child) in self.root.children.iter() {
            let node_winrate = child.wins as f64 / child.played as f64;
            if node_winrate > best_winrate {
                best_winrate = node_winrate;
                best_moves = vec![move_];
            } else if node_winrate == best_winrate {
                best_moves.push(move_);
            }
        }

        let mut rng = thread_rng();
        let &best_move = best_moves.choose(&mut rng).unwrap();
        let new_root = self.root.children.remove(&best_move).unwrap();
        self.root = new_root;

        best_move
    }
}
/*
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
}*/