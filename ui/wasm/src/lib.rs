use wasm_bindgen::prelude::*;

use mcts::{hex, search};
use mcts::Game;

#[wasm_bindgen]
pub struct Client {
    state: hex::Hex,
    tree: search::SearchTree<hex::Hex>,
}

#[wasm_bindgen]
impl Client {
    pub fn new(n: usize) -> Self {
        Self {
            state: hex::Hex::new(n),
            tree: search::SearchTree::new(),
        }
    }

    pub fn play(&mut self, action: u32) {
        self.state = self.state.next_state(action);
    }

    pub fn think(&mut self, its: usize) -> u32 {
        for _ in 0..its {
            self.tree.search(&self.state);
        }
        self.tree.action_prob(&self.state)
    }

    pub fn terminal_value(&self) -> Option<f32> {
        self.state.terminal_value()
    }
}


