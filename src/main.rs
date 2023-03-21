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

use player::{Player, Strategy};
fn main() {
    let mut player = Player::new(Some(Strategy::Mixed));
    player.play_games(100000);
    print!("done");
    /*let mut turn = board::Case::Black;
    let mut board = board::Board::new();
    use crate::minimax::calculate_outcomes;
    let a = calculate_outcomes(&board, &turn, 3);
    for b in a.iter() {
        println!("{:?}", b);
    }*/
}
