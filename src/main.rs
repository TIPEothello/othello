/*
 File: main.rs
 Created Date: 21 Mar 2023
 Author: realbacon
 -----
 Last Modified: 2/04/2023 01:18:5
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
    use crate::minimax::calculate_outcomes;
    let mut bl = 0;
    let mut wh = 0;
    let mut dr = 0;
    for _ in 0..7000 {
        let mut board = board::Board::new();
        loop {
            //println!("{}", board);
            if board.available_moves(None).len() == 0 {
                break;
            }

            let outcomes = calculate_outcomes(&board, 2);
            let best_move = minimax::minimax(&outcomes, &mut board);
            //println!("Black plays");
            board.make_move(&best_move).unwrap();
            //println!("{}", board);
            if board.available_moves(None).len() == 0 {
                break;
            }
            //println!("{}", board);
            //println!("White plays");
            board.make_move_with_highest_gain().unwrap();
            //println!("{}", board);
        }
        if board.score().0 > board.score().1 {
            wh += 1;
        } else if board.score().0 < board.score().1 {
            bl += 1;
        } else {
            dr += 1;
        }
    }
    println!("{:?}", (bl, wh, dr));
}
