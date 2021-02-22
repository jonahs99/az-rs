use std::collections::HashMap;
use std::hash::Hash;
use rand::Rng;
use ordered_float::OrderedFloat;

use crate::Game;

pub struct SearchTree<G: Hash> {
    nodes: HashMap<G, Node>,
}

enum Node {
    Terminal {
        value: f32,
    },
    Inner {
        n: f32,
        stats: HashMap<u32, Stats>,
    },
}

struct Stats {
    v: f32,
    n: f32,
}

impl <G: Eq + Hash + Game + Clone> SearchTree<G> {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    /// Action probablity estimate from the given state based on previous search iterations
    pub fn action_prob(&self, s: &G) -> u32 {
        let node = self.nodes.get(s);
        match node {
            Some(Node::Inner{ stats, .. }) => {
                let actions = s.valid_actions();
                actions.max_by_key(|a| match stats.get(a) {
                    Some(Stats { n, .. }) => OrderedFloat(*n),
                    None => OrderedFloat(0.),
                }).expect("No outgoing actions")
            },
            Some(Node::Terminal{ .. }) => panic!("The state is terminal"),
            None => panic!("The state has not been searched"),
        }
    }

    /// Runs an iteration of Monte-Carlo Tree Search
    pub fn search(&mut self, s: &G) -> f32 {
        let mut leaf_value = None;
        let node = self.nodes.entry(s.clone()).or_insert_with(|| {
            if let Some(value) = s.terminal_value() {
                Node::Terminal{ value }
            } else {
                // Inserting a leaf
                leaf_value = Some(rollout(s.clone()));
                Node::Inner{
                    n: 0.,
                    stats: HashMap::new(),
                }
            }
        });

        if let Some(v) = leaf_value {
            return -v;
        }

        let cpuct = 2.0_f32.sqrt();
        let action = match node {
            Node::Terminal{ value } => { return -*value; },
            Node::Inner{ n: n_s, stats } => {
                // Pick the action with the highest upper confidence bound
                let actions = s.valid_actions();
                actions.max_by_key(|a|
                    match stats.get(a) {
                        Some(Stats { v, n: n_sa }) =>
                            OrderedFloat((*v / *n_sa) + cpuct * ( *n_s / *n_sa ).sqrt()),
                        None =>
                            OrderedFloat(std::f32::INFINITY),
                    }
                ).unwrap()
            },
        };

        let next_state = s.next_state(action);
        let v = self.search(&next_state);

        if let Some(Node::Inner{ n, stats }) = self.nodes.get_mut(s) {
            let stats = stats.entry(action).or_insert(Stats{ v: 0., n: 0. });
            stats.v += v;
            stats.n += 1.;
            *n += 1.;
        }
        
        -v
    }
}

fn select_action<G: Game>(state: &G) -> u32 {
    let mut rng = rand::thread_rng();
    let actions_count = state.valid_actions_count();
    state.valid_actions().nth(rng.gen_range(0..actions_count)).unwrap()
}

fn rollout<G: Game>(start: G) -> f32 {
    let mut state = start;
    let mut player = 1.;
    while state.terminal_value().is_none() {
        state = state.next_state(select_action(&state));
        player *= -1.;
    }
    state.terminal_value().unwrap() * player
}

