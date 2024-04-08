use crate::{
    error::{Error, Result},
    state::State,
};

use super::{
    actions::{ChangeContextClosure, QueryNextClosure},
    choice::Choice,
    traits::Progressive,
};

#[derive(Debug)]
pub struct Dialogue {
    text: String,
    character: String,
    choices: Vec<Choice>,
    query_next: Option<QueryNextClosure>,
    change_context: Option<ChangeContextClosure>,
}

impl Dialogue {
    pub fn text(&self) -> &String {
        &self.text
    }
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }
    pub fn character(&self) -> &String {
        &self.character
    }
    pub fn set_character(&mut self, character: impl Into<String>) {
        self.character = character.into();
    }
    pub fn choices(&self) -> &Vec<Choice> {
        &self.choices
    }
    pub fn mut_choices(&mut self) -> &mut Vec<Choice> {
        &mut self.choices
    }
    pub fn query_next(&self) -> Option<&QueryNextClosure> {
        self.query_next.as_ref()
    }
    pub fn set_query_next(&mut self, closure: QueryNextClosure) {
        self.query_next = Some(closure);
    }
    pub fn change_context(&self) -> Option<&ChangeContextClosure> {
        self.change_context.as_ref()
    }
    pub fn set_change_context(&mut self, closure: ChangeContextClosure) {
        self.change_context = Some(closure);
    }
}

#[derive(Debug)]
pub struct DialogueBuilder {
    text: String,
    character: String,
    choices: Vec<Choice>,
    query_next: Option<QueryNextClosure>,
    change_context: Option<ChangeContextClosure>,
}

#[derive(Debug)]
pub struct DialogueLayout<'a> {
    pub text: &'a str,
    pub character: &'a str,
}

impl DialogueBuilder {
    pub fn new(layout: DialogueLayout) -> Self {
        Self {
            text: layout.text.to_string(),
            character: layout.character.to_string(),
            choices: Vec::new(),
            query_next: None,
            change_context: None,
        }
    }
    pub fn add_choice(mut self, choice: impl Into<Choice>) -> Self {
        self.choices.push(choice.into());
        self
    }
    pub fn set_query_next(mut self, closure: QueryNextClosure) -> Self {
        self.query_next = Some(closure);
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
            query_next: self.query_next,
            change_context: self.change_context,
        }
    }
}

impl From<DialogueBuilder> for Dialogue {
    fn from(value: DialogueBuilder) -> Self {
        Self {
            text: value.text,
            character: value.character,
            choices: value.choices,
            query_next: value.query_next,
            change_context: value.change_context,
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
            match self.query_next {
                Some(next_closure) => Ok(Some(next_closure(state.context()))),
                None => Ok(None),
            }
        } else {
            let choice = match choice_index {
                Some(choice_index) => match self.choices.get(choice_index) {
                    Some(choice) => choice,
                    None => {
                        return Err(Error::InvalidChoice {
                            index: choice_index,
                        })
                    }
                },
                None => return Err(Error::ChoiceWasNotProvided),
            };
            Ok(choice.next(state, choice_index))
        }
    }
}
