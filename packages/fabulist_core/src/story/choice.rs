use crate::state::State;

use super::{
    actions::{ChangeContextClosure, NextClosure},
    traits::Progressive,
};

pub struct Choice {
    pub text: String,
    pub response: Option<String>,
    pub next: Option<NextClosure>,
    pub change_context: Option<ChangeContextClosure>,
}

pub struct ChoiceBuilder {
    text: String,
    response: Option<String>,
    next: Option<NextClosure>,
    change_context: Option<ChangeContextClosure>,
}

impl ChoiceBuilder {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            response: None,
            next: None,
            change_context: None,
        }
    }
    pub fn set_response(mut self, response: String) -> Self {
        self.response = Some(response);
        self
    }
    pub fn set_next(mut self, closure: NextClosure) -> Self {
        self.next = Some(closure);
        self
    }
    pub fn set_change_context(mut self, closure: ChangeContextClosure) -> Self {
        self.change_context = Some(closure);
        self
    }
    pub fn build(self) -> Choice {
        Choice {
            text: self.text,
            response: self.response,
            next: self.next,
            change_context: self.change_context,
        }
    }
}

impl Progressive for Choice {
    type Output = Option<String>;
    fn next(&self, state: &mut State, _choice_index: Option<usize>) -> Self::Output {
        match self.change_context {
            Some(change_context_closure) => {
                change_context_closure(state.mut_context());
            }
            None => (),
        }
        match self.next {
            Some(next_closure) => Some(next_closure(state.context())),
            None => None,
        }
    }
}
