mod board;
mod mcts;
mod minimax;
mod player;
mod rules;

#[tokio::main(worker_threads = 8, flavor = "multi_thread")]
async fn main() {
    let mut player = player::Player::new((
        player::Strategy::Minimax { depth: 6 },
        player::Strategy::MCTS { playout_budget: 60000 },
    ));
    player.progressive_play();
}
