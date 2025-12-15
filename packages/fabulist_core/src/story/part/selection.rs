use fabulist_derive::ElementInternal;

use crate::{
    error::{EngineError, EngineResult},
    story::{reference::ListKey, Progressive},
};

use super::{choice::Choice, PartElement};

#[derive(ElementInternal, Debug)]
pub struct Selection(Vec<Choice>);

impl Selection {
    pub fn choices(&self) -> &Vec<Choice> {
        &self.0
    }

    pub fn mut_choices(&mut self) -> &mut Vec<Choice> {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct SelectionBuilder(Vec<Choice>);

impl Default for SelectionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SelectionBuilder {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add_choice(mut self, choice: impl Into<Choice>) -> Self {
        self.0.push(choice.into());
        self
    }

    pub fn build(self) -> Selection {
        Selection(self.0)
    }
}

impl From<SelectionBuilder> for Selection {
    fn from(value: SelectionBuilder) -> Self {
        Self(value.0)
    }
}

impl From<SelectionBuilder> for Box<PartElement> {
    fn from(value: SelectionBuilder) -> Self {
        Box::new(Selection(value.0))
    }
}

impl Progressive for Selection {
    type Output = EngineResult<Option<ListKey<String>>>;
    fn next(&self, state: &mut crate::state::State, choice_index: Option<usize>) -> Self::Output {
        if !self.0.is_empty() {
            let choice = match choice_index {
                Some(choice_index) => match self.0.get(choice_index) {
                    Some(choice) => choice,
                    None => {
                        return Err(EngineError::InvalidChoice {
                            index: choice_index,
                        })
                    }
                },
                None => return Err(EngineError::ChoiceWasNotProvided),
            };
            choice.next(state, choice_index)
        } else {
            Ok(None)
        }
    }
}
