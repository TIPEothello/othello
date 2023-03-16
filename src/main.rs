mod board;
mod rules;
mod player;

use player::{Player, Strategy};
fn main() {
	let mut player = Player::new(Some(Strategy::Mixed));
	let (white_wins, black_wins, draws) = player.play_games(15000);
	println!("White wins: {}, Black wins: {}, Draws: {}", white_wins, black_wins, draws);
	println!("Efficiency: {}%", (black_wins as f32) * 100.0 / (white_wins + black_wins + draws) as f32);
}