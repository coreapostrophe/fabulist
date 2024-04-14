use crate::{
    error::{Error, Result},
    story::{resource::InterpInset, Progressive},
};

use super::{choice::Choice, Element, PartElement};

#[derive(Debug)]
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

impl InterpInset for Selection {
    fn interp_inset(&mut self, _resource: &mut crate::story::resource::Resources) {}
}

impl Element for Selection {}

impl Progressive for Selection {
    type Output = Result<Option<String>>;
    fn next(&self, state: &mut crate::state::State, choice_index: Option<usize>) -> Self::Output {
        if !self.0.is_empty() {
            let choice = match choice_index {
                Some(choice_index) => match self.0.get(choice_index) {
                    Some(choice) => choice,
                    None => {
                        return Err(Error::InvalidChoice {
                            index: choice_index,
                        })
                    }
                },
                None => return Err(Error::ChoiceWasNotProvided),
            };
            choice.next(state, choice_index)
        } else {
            Ok(None)
        }
    }
}
