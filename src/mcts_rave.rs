#![allow(non_snake_case)]

use core::panic;
use std::collections::HashMap;

use crate::board::{Board, BoardState, Case, EndState};
use fxhash::FxHashMap;
use rand::{seq::SliceRandom, thread_rng};

const EXPLORATION_PARAMETER: f64 = std::f64::consts::SQRT_2;
const RAVE_CONST: f64 = 750.0;

#[derive(Debug, Clone)]
struct Node {
    played: u64,
    reward: i64, // wins - losses
    turn: Case,
    state: Board,
    children: FxHashMap<(usize, usize), Node>,
    is_fully_expanded: bool,
    reward_rave: i64,
    played_rave: u64,
    exploration_constant: f64,
    winning_state: Option<Case>,
}

impl Node {
    fn from_expansion(parent: &Node, mut moves: Vec<(usize, usize)>, move_: (usize, usize)) -> (Node, EndState, Vec<(usize, usize)>) {
        let mut board = parent.state.clone();
        let player = parent.turn.opponent();
        let (is_end_state, (endstate, mut movs)) = match board.play_move(&move_) {
            Ok(BoardState::Ongoing) => (
                false,
                Node::simulate_random_playout(&mut board.clone(), player),
            ),
            Ok(BoardState::Ended(endstate)) => (true, (endstate, vec![])),
            Err(msg) => panic!(
                "error in Node.from_expansion when calling board.play_move(): {}",
                msg
            ),
        };
        moves.append(&mut movs);
        let mut node = Node {
            state: board,
            turn: player,
            is_fully_expanded: is_end_state,
            played: 0,
            played_rave: 0,
            reward_rave: 0,
            reward: 0,
            children: HashMap::default(),
            exploration_constant: parent.exploration_constant,
            winning_state: None,
        };

        node.update_from_endstate(endstate);

        (node, endstate, moves)
    }

    fn simulate_random_playout(
        board: &mut Board,
        player: Case,
    ) -> (EndState, Vec<(usize, usize)>) {
        let mut curr_player = player;
        let mut rng = thread_rng();
        let mut moves: Vec<(usize, usize)> = Vec::new();
        loop {
            let move_ = board.available_moves(None).choose(&mut rng).unwrap().clone();
            let game_state = board.play_move(&move_);
            moves.push(move_.clone());
            curr_player = curr_player.opponent();
            match game_state {
                Ok(state) => match state {
                    BoardState::Ongoing => (),
                    BoardState::Ended(endstate) => return (endstate, moves),
                },
                Err(msg) => panic!("Err in Node.simulate_random_playout(): {}", msg),
            }
        }
    }

    fn update_from_endstate(&mut self, endstate: EndState) -> () {
        self.played += 1;
        let EndState::Winner(winner) = endstate;
        if winner == self.turn {
            self.reward += 1;
        } else if winner == self.turn.opponent() {
            self.reward -= 1;
        }
    }

    fn update_from_endstate_rave(&mut self, endstate: EndState, moves: &Vec<(usize, usize)>) -> () {
        let EndState::Winner(winner) = endstate;
        let reward: i64 = if winner == self.turn {
            1
        } else if winner == self.turn.opponent() {
            -1
        } else {
            0
        };
        for i in moves {
            if let Some(c) = self.children.get_mut(&i) {
                c.played_rave += 1;
                c.reward_rave += reward;
            }
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
        
        let rave = |child: &Node, parent: &Node| {
            let alpha = f64::sqrt(RAVE_CONST / (3.0 * (child.played as f64) + RAVE_CONST));
            let uct =  (child.reward as f64 / child.played as f64) + exploration_constant * f64::sqrt(f64::ln(parent.played as f64) / child.played as f64);
            let amaf = if child.played_rave == 0 { 0.0_f64 } else { (child.reward_rave as f64)/(child.played_rave as f64) };
            (1.0 - alpha) * uct + alpha * amaf
        };


        match self.children.get(move_) {
            Some(child) => {
                if child.is_fully_expanded {
                    0.
                } else if child.played == 0 {
                    f64::INFINITY
                } else {
                    rave(child, self)
                }
            }
            None => unreachable!(),
        }
    }

    //Un round d'expansion
    fn expand(&mut self, movs: Vec<(usize, usize)>) -> (EndState, Vec<(usize, usize)>) {
        if self.is_fully_expanded {
            return (EndState::Winner(Case::Empty), movs);
        }

        let mut moves = self.state.available_moves(None);
        for move_ in moves.iter() {
            if !self.children.contains_key(move_) {
                let (child_node, endstate, mut movs) = Node::from_expansion(self, movs, *move_);
                movs.push(*move_);
                self.children.insert(*move_, child_node); // Ajoute le move aux children
                self.update_from_endstate(endstate); // Si le move est terminal on update les scores
                self.update_from_endstate_rave(endstate, &movs);
                self.update_fully_expanded(); // Si tous les children sont fully expanded on le note comme fully expanded
                return (endstate, movs);
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

        while let Some(best_move) = moves.pop() {
            let child = self.children.get_mut(&best_move).unwrap();
            if !child.is_fully_expanded {
                
                let (endstate, mut movs) = child.expand(movs); //On expand le meilleur move qui n'est pas encore fully expanded
                movs.push(best_move);
                self.update_from_endstate(endstate);
                self.update_from_endstate_rave(endstate, &movs);
                self.update_fully_expanded();
                return (endstate, movs);
            }
        }

        panic!("Expand is broken !")
    }

    pub fn update_winning_state(turn: Case, current: Case, to_add: Case) -> Case {
        if turn != current {
            if to_add != turn.opponent() {
                return to_add;
            }
            return current;
        }
        current
    }

    pub fn generate_winning_state(&mut self) -> () {
        // Algorithme de recherche d'attracteur
        if self.state.is_ended() {
            self.winning_state = Some(self.state.current_winner());
        } else {
            let mut wstate = self.turn;
            for node in self.children.values_mut() {
                node.generate_winning_state();
                wstate = Node::update_winning_state(
                    self.turn.opponent(),
                    wstate,
                    node.winning_state.unwrap(),
                );
            }
            self.winning_state = Some(wstate);
        }
    }
}

#[derive(Debug, Clone)]
pub struct MCTSRave {
    root: Node,
    playout_budget: usize,
    final_solve: bool,
}

impl MCTSRave {
    pub fn new(player: Case, final_solve: bool, playout_budget: usize, board: Board) -> MCTSRave {
        let root = Node {
            state: board,
            turn: player.opponent(),
            is_fully_expanded: false,
            played: 0,
            reward: 0,
            reward_rave: 0,
            played_rave: 0,
            children: HashMap::default(),
            exploration_constant: EXPLORATION_PARAMETER,
            winning_state: None,
        };

        MCTSRave {
            playout_budget,
            root,
            final_solve,
        }
    }

    pub fn search(&mut self, board: &Board) -> (usize, usize) {
        if let Some(opp_move) = self.get_opponents_last_move(board) {
            self.update_with_opponents_move(opp_move, board);
        }
        let move_ = {
            if self.root.is_fully_expanded && self.final_solve {
                if self.root.winning_state.is_none() {
                    self.root.generate_winning_state();
                    //println!("[MCTS] Detected Winner : {}", self.root.winning_state.unwrap());
                }
                let mut rng = thread_rng();
                let mut moves = Vec::new();

                for (m, n) in &self.root.children {
                    if n.winning_state.unwrap() == self.root.winning_state.unwrap() {
                        moves.push((m.clone(), n));
                    }
                }
                let (m, _) = *moves.choose(&mut rng).unwrap();
                m
            } else {
                for _ in 0..self.playout_budget {
                    self.root.expand(vec![]);
                }

                self.get_best_move()
            }
        };

        //Détection d'erreurs :

        let available = self.root.state.available_moves(None);
        if !available.contains(&move_) {
            println!("[MCTSRave] Move foud that doesn't work : {:?}", &move_);
            println!("Available moves vs children :");
            println!("{:?}", available);
            println!("{:?}", self.root.children.keys().collect::<Vec<&(usize, usize)>>())
        }
        
        self.promote_child(&move_);
        move_
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

    fn get_best_move(&mut self) -> (usize, usize) {
        let mut best_winrate = std::i64::MIN;
        let mut best_moves: Vec<(usize, usize)> = vec![];

        for (&move_, child) in self.root.children.iter() {
            let node_winrate = child.reward;
            if node_winrate > best_winrate {
                best_winrate = node_winrate;
                best_moves = vec![move_];
            } else if node_winrate == best_winrate {
                best_moves.push(move_);
            }
        }

        let mut rng = thread_rng();
        let &best_move = best_moves.choose(&mut rng).unwrap();
        best_move
    }

    fn promote_child(&mut self, move_: &(usize, usize)) -> () {
        let new_root = self.root.children.remove(move_).unwrap();
        self.root = new_root;
    }
}

#[test]
fn update_winning_state_test() {
    let turn = Case::White;
    let mut current = turn.opponent();
    let states = [
        Case::Black,
        Case::Black,
        Case::Empty,
        Case::White,
        Case::Black,
        Case::Empty,
        Case::White,
    ];
    let expected = [
        Case::Black,
        Case::Black,
        Case::Empty,
        Case::White,
        Case::White,
        Case::White,
        Case::White,
    ];
    for i in 0..7 {
        current = Node::update_winning_state(turn, current, states[i]);
        assert_eq!(current, expected[i])
    }
}
