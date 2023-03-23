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
#![allow(dead_code)]

use crate::board::{Board};
/// A function  that takes a board and a turn and returns a vector of all possible outcomes
/// # Arguments
/// * `board` - The board to calculate the outcomes from
/// * `turn` - The turn to calculate the outcomes for
///
pub fn calculate_outcomes(board: &Board, depth: i8) -> Vec<Vec<(usize, usize)>> {
    let mut board = board.clone();
    fn calculate_outcomes_rec(
        board: &mut Board,
        depth: i8,
        played_moves: Vec<(usize, usize)>,
    ) -> Vec<Vec<(usize, usize)>> {
		let mut outcomes = Vec::new();
		board.play_moves(&played_moves).unwrap();
		for moves in board.available_moves(None) {
			let mut new_moves = played_moves.clone();
			new_moves.push(moves);
			if depth == 0 {
				outcomes.push(new_moves);
			} else {
				board.reset(played_moves.len());
				outcomes.append(&mut calculate_outcomes_rec(board, depth - 1, new_moves));
			}
		}
		board.reset(played_moves.len());
		outcomes
    }
    calculate_outcomes_rec(&mut board, depth - 1, Vec::new())
}
