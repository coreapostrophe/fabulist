use std::fmt::Debug;

use crate::state::State;

pub trait Progressive: Debug {
    type Output;
    fn next(&self, state: &mut State, choice_index: Option<usize>) -> Self::Output;
}

pub trait Element: Progressive {}

pub trait Keyed {
    fn id(&self) -> &String;
}
