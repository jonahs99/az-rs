use mcts::{hex, search};
use mcts::Game;

fn main() {
    let mut state = hex::Hex::new(5);

    let mut tree = search::SearchTree::new();

    println!("{}", &state);
    while state.terminal_value().is_none() {
        for _ in 0..1000 {
            tree.search(&state);
        }
        state = state.next_state(tree.action_prob(&state));
        println!("{}", &state);
    }
}

