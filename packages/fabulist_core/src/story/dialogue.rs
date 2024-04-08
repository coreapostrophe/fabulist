use crate::{
    error::{Error, Result},
    state::State,
};

use super::{
    actions::{ChangeContextClosure, NextClosure},
    choice::Choice,
    traits::Progressive,
};

pub struct Dialogue {
    pub text: String,
    pub character: String,
    pub choices: Vec<Choice>,
    pub next: Option<NextClosure>,
    pub change_context: Option<ChangeContextClosure>,
}

pub struct DialogueBuilder {
    pub text: String,
    pub character: String,
    pub choices: Vec<Choice>,
    pub next: Option<NextClosure>,
    pub change_context: Option<ChangeContextClosure>,
}

pub struct DialogueLayout {
    pub text: String,
    pub character: String,
}

impl DialogueBuilder {
    pub fn new(layout: DialogueLayout) -> Self {
        Self {
            text: layout.text,
            character: layout.character,
            choices: Vec::new(),
            next: None,
            change_context: None,
        }
    }
    pub fn add_choice(mut self, quote: Choice) -> Self {
        self.choices.push(quote);
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
    pub fn build(self) -> Dialogue {
        Dialogue {
            text: self.text,
            character: self.character,
            choices: self.choices,
            next: self.next,
            change_context: self.change_context,
        }
    }
}

impl Progressive for Dialogue {
    type Output = Result<Option<String>>;
    fn next(&self, state: &mut State, choice_index: Option<usize>) -> Self::Output {
        if self.choices.is_empty() {
            match self.change_context {
                Some(change_context_closure) => {
                    change_context_closure(state.mut_context());
                }
                None => (),
            }
            match self.next {
                Some(next_closure) => Ok(Some(next_closure(state.context()))),
                None => Ok(None),
            }
        } else {
            let choice = match choice_index {
                Some(choice_index) => match self.choices.get(choice_index) {
                    Some(choice) => choice,
                    None => return Err(Error::InvalidChoice { choice_index }),
                },
                None => return Err(Error::ChoiceWasNotProvided),
            };
            Ok(choice.next(state, choice_index))
        }
    }
}
