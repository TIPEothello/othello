// Monte Carlo Tree Search

use rand::Rng;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::AddAssign;

// Implement Monte Carlo Tree Search
#[derive(Debug)]
pub struct MCTS {
    tree : Node
}

#[derive(Debug)]
struct Node {
    wins: i64,
    played : i64,
    leafs : Vec<Node>
}

impl MCTS {
    pub fn new() -> Self {
        MCTS {
            tree : Node {
                wins : 0,
                played : 0,
                leafs : Vec::new()
            }
        }
    }
}

#[test]
fn create_mcts() {
    let mcts = MCTS::new();
    println!("{:?}",mcts);
}

