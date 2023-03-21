/*
 File: strategy.rs
 Created Date: 21 Mar 2023
 Author: realbacon
 -----
 Last Modified: 21/03/2023 03:20:14
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

use crate::board::{Board, Case};
use crate::rules::enemy;
/// A function  that takes a board and a turn and returns a vector of all possible outcomes
/// # Arguments
/// * `board` - The board to calculate the outcomes from
/// * `turn` - The turn to calculate the outcomes for
///
pub fn calculate_outcomes(board: &Board, turn: &Case, depth: i8) -> Vec<Vec<(usize, usize)>> {
    let mut board = board.clone();
    fn calculate_outcomes_rec(
        board: &mut Board,
        turn: &Case,
        depth: i8,
        played_moves: Vec<(usize, usize)>,
    ) -> Vec<Vec<(usize, usize)>> {
        let mut outcomes = Vec::new();
        board.play_moves(&played_moves).unwrap();
        println!("{}", board);
        println!("{:?}", board.available_moves(turn));
        for moves in board.available_moves(turn) {
            let mut new_moves = played_moves.clone();
            println!("{} ", turn);
            new_moves.push(moves);
            board.reset(new_moves.len() - 1);
            println!("reset {}", new_moves.len() - 1);
            board.play_moves(&played_moves).unwrap();
            println!("played {}", played_moves.len());
            if depth > 0 {
                let mut new_outcomes =
                    calculate_outcomes_rec(board, &enemy(turn), depth - 1, new_moves);
                outcomes.append(&mut new_outcomes);
            } else {
                outcomes.push(new_moves);
            }
        }
        outcomes
    }
    calculate_outcomes_rec(&mut board, turn, depth - 1, Vec::new())
}
