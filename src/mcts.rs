#![allow(non_snake_case)]

use core::panic;
use std::collections::HashMap;

use crate::board::{Board, BoardState, Case, EndState};
use fxhash::FxHashMap;
use rand::{seq::SliceRandom, thread_rng};

const EXPLORATION_PARAMETER: f64 = std::f64::consts::SQRT_2;

#[derive(Debug, Clone)]
struct Node {
    played: u64,
    wins: u64,
    turn: Case,
    state: Board,
    children: FxHashMap<(usize, usize), Node>,
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
                Node::simulate_random_playout(&mut board.clone(), player),
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
            is_fully_expanded: is_end_state,
            played: 0,
            wins: 0,
            children: HashMap::default(),
            exploration_constant: parent.exploration_constant,
        };

        node.update_from_endstate(endstate);

        (node, endstate)
    }

    fn simulate_random_playout(board: &mut Board, player: Case) -> EndState {
        let mut curr_player = player;
        loop {
            let mut rng = thread_rng();
            let game_state = board.play_move(board.available_moves(None).choose(&mut rng).unwrap());
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
        let calculate_uct = |score: f64, c_visits: u64, p_visits: u64| {
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
            None => unreachable!(),
        }
    }

    //Un round d'expansion
    fn expand(&mut self) -> EndState {
        if self.is_fully_expanded {
            return EndState::Winner(Case::Empty);
        }

        let mut moves = self.state.available_moves(None);
        for move_ in moves.iter() {
            if self.children.get(&move_).is_none() {
                let (child_node, endstate) = Node::from_expansion(self, *move_);
                self.children.insert(*move_, child_node); // Ajoute le move aux children
                self.update_from_endstate(endstate); // Si le move est terminal on update les scores
                self.update_fully_expanded(); // Si tous les children sont fully expanded on le note comme fully expanded
                return endstate;
            }
        }
        // Si on arrive là, tous les children ont déjà été visités
        // donc il faut aller un niveau plus loin (en passant par le move choisi par l'UCT)

        moves.sort_by(|m1, m2| {
            //On sort les moves par UCT
            self.get_exploration_score(self.exploration_constant, m1)
                .partial_cmp(&self.get_exploration_score(self.exploration_constant, m2))
                .unwrap()
        });

        while !moves.is_empty() {
            let best_move = moves.pop().unwrap();
            let child = self.children.get_mut(&best_move.into()).unwrap();
            if !child.is_fully_expanded {
                let endstate = child.expand(); //On expand le meilleur move qui n'est pas encore fully expanded
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
    playout_budget: usize,
}

impl MCTS {
    pub fn new(player: Case, playout_budget: usize, board: Board) -> MCTS {
        let root = Node {
            state: board,
            turn: player.opponent(),
            is_fully_expanded: false,
            played: 0,
            wins: 0,
            children: HashMap::default(),
            exploration_constant: EXPLORATION_PARAMETER,
        };

        MCTS {
            playout_budget,
            root,
        }
    }

    pub fn search(&mut self, board: &Board) -> (usize, usize) {
        if let Some(opp_move) = self.get_opponents_last_move(board) {
            self.update_with_opponents_move(opp_move);
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

    fn update_with_opponents_move(&mut self, opp_move: (usize, usize)) {
        match self.root.children.remove(&opp_move.into()) {
            Some(child_node) => self.root = child_node,
            None => unreachable!(),
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
        let new_root = self.root.children.remove(&best_move.into()).unwrap();
        self.root = new_root;

        best_move
    }
}
