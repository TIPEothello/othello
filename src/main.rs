/*
 File: main.rs
 Created Date: 21 Mar 2023
 Author: realbacon
 -----
 Last Modified: 21/03/2023 03:01:54
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

mod board;
mod minimax;
mod player;
mod rules;
fn main() {
    let board = board::Board::new();
    use crate::minimax::calculate_outcomes;
    for n in 1..=10 {
		let outcomes = calculate_outcomes(&board, n);
		println!("Depth : {} / Outcomes: {}", n, outcomes.len());
	}
}
