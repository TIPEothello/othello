mod board;
mod mcts;
mod minimax;
mod player;
mod rules;

fn main() {
    let mut player = player::Player::new((
        player::Strategy::Minimax { depth: 4 },
        player::Strategy::MCTS {
            playout_budget: 4000,
        },
    ));
    player.play_games(100, true);
}
