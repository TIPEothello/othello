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
    let mut player = player::Player::new(Some((player::Strategy::Minimax { depth: 2 }, player::Strategy::Random)));
	println!("{:?}",player.play_games(1000));
}
