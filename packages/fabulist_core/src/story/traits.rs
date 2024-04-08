use crate::state::State;

pub trait Progressive {
    type Output;
    fn next(&self, state: &mut State, choice_index: Option<usize>) -> Self::Output;
}
