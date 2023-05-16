/*
 File: main.rs
 Created Date: 21 Mar 2023
 Author: realbacon
 -----
 Last Modified: 23/04/2023 05:43:58
 Modified By: realbacon
 -----
 License  : MIT
 -----
*/

mod board;
mod minimax;
mod player;
mod rules;
mod scraper;
mod mcts;

#[tokio::main(worker_threads = 100)]
async fn main() {
    
    



    let mut player = player::Player::new(Some((
        player::Strategy::Greedy,
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

     
}
/* 
use minimax::CoefS;
async fn gradiant_descent() {
    let step = 0.6;
    let h = 0.0001;
    let mut player = player::Player::new(Some((
        player::Strategy::Minimax { depth: 4 },
        player::Strategy::MinimaxTree { depth: 4 },
    )));
    for _ in 0..40 {
        let mut coef = CoefS::from_file("coef.json");
        let init_res = player.play_games(400).await;
        // Modify the file coef.json
        let ini = coef.material;
        coef.material += h;
        coef.modify_file("coef.json").unwrap();
        let new_res = player.play_games(400).await;
        let der = (new_res.1 - init_res.1) as f32 / h;
        coef.material = ini - step * der;
    }
}
*/