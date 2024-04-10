use std::fmt::Debug;

use crate::state::State;

pub trait Progressive: Debug {
    type Output;
    fn next(&self, state: &mut State, choice_index: Option<usize>) -> Self::Output;
}

pub type ProgressiveElement<T> = dyn Progressive<Output = T>;
