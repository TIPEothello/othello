mod board;
mod mcts;
mod minimax;
mod player;
mod rules;

#[tokio::main(worker_threads = 3, flavor = "multi_thread")]
async fn main() {
    let mut player = player::Player::new((
        player::Strategy::MCTS {
            playout_budget: 5000,
        },
        player::Strategy::Minimax { depth: 4 },
    ));
    println!("{:?}", player.play_games(100).await);
}
