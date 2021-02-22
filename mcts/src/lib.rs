#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]

pub mod search;
pub mod hex;

type Action = u32;
type Value = f32;

/// A 2-player adversarial turn-based game (discrete steps with finite actions)
pub trait Game {
    type ActionIter<'a>: Iterator<Item=Action> + 'a;
    
    /// Iterator over the valid actions the player can take from this state
    fn valid_actions(&self) -> Self::ActionIter<'_>;

    /// The state after taking an action. Must be a valid action for this state
    fn next_state(&self, action: Action) -> Self;

    /// The value if this is a terminal state
    fn terminal_value(&self) -> Option<Value>;

    /// The number of valid actions from this state. Must be the length of the valid_actions
    /// iterator
    fn valid_actions_count(&self) -> usize {
        self.valid_actions().count()
    }
}

