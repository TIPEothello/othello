/*
 File: main.rs
 Created Date: 14 Mar 2023
 Author: realbacon
 -----
 Last Modified: 14/03/2023 10:46:13
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/


mod board;
mod rules;

use board::Board;
use board::Case;
use rand::seq::SliceRandom;
use rules::enemy;

fn main() {
    let mut rng = rand::thread_rng();
    let mut board = Board::new();
    let mut turn = Case::Black;
    while board.available_moves(&turn).len() > 0 {
        println!("{}", board);
        // choose a random move within the available moves
        let bmove = *board.available_moves(&turn).choose(&mut rng).unwrap();
        board.make_move(bmove, turn).unwrap();
        turn = enemy(&turn);
    }

    println!("{}", board);

    let (white, black) = board.score();
    println!("White: {}, Black: {}", white, black);
    println!(
        "Winner: {}",
        if white > black {
            "White"
        } else if black > white {
            "Black"
        } else {
            "Draw"
        }
    );
}
