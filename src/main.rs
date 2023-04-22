/*
 File: main.rs
 Created Date: 21 Mar 2023
 Author: realbacon
 -----
 Last Modified: 19/04/2023 05:17:33
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

mod board;
mod minimax;
mod player;
mod rules;

#[tokio::main(worker_threads = 100)]
async fn main() {

	/*let mut player = player::Player::new(Some((
		player::Strategy::Greedy,
		player::Strategy::MinimaxTree { depth: 4 },
		
	)));

	player.play_games(100)*/

    let mut player = player::Player::new(Some((
        player::Strategy::Minimax { depth: 4 },
        player::Strategy::MinimaxTree { depth: 4 },
    )));
    //player.progressive_play();

    let result = player.play_games(100).await;
    println!(
        "Win ratio : White {}% ({}) - Black {}% ({})",
        result.0 as f32 / (result.0 + result.1 + result.2) as f32 * 100.0,
        result.0,
        result.1 as f32 / (result.0 + result.1 + result.2) as f32 * 100.0,
        result.1
    );
    /*
     let mut board = board::Board::new();
    for _ in 0..4 {
        board.make_move(&board.available_moves(None)[0]).unwrap();
    }
    let mut tree = minimax::Tree::from_board(&mut board.clone(), None, 4);
    minimax::minimax_tree(&mut tree, board::Case::Black);*/
}
