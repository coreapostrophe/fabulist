use crate::{
    error::{Error, Result},
    state::State,
};

use super::{
    traits::{Progressive, ProgressiveElement},
    DialogueIndex,
};

pub mod actions;
pub mod choice;
pub mod choices;
pub mod dialogue;

pub type Quote = ProgressiveElement<Result<Option<String>>>;

#[derive(Debug)]
pub struct Part {
    id: String,
    quotes: Vec<Box<Quote>>,
}

impl Part {
    pub fn id(&self) -> &String {
        &self.id
    }
    pub fn quotes(&self) -> &Vec<Box<Quote>> {
        &self.quotes
    }
    pub fn mut_quotes(&mut self) -> &mut Vec<Box<Quote>> {
        &mut self.quotes
    }
    pub fn quote(&self, index: usize) -> Result<&Box<Quote>> {
        match self.quotes.get(index) {
            Some(dialogue) => Ok(dialogue),
            None => {
                return Err(Error::DialogueDoesNotExist {
                    dialogue_index: index,
                    part_key: self.id.clone(),
                })
            }
        }
    }
    pub fn mut_quote(&mut self, index: usize) -> Result<&mut Box<Quote>> {
        match self.quotes.get_mut(index) {
            Some(dialogue) => Ok(dialogue),
            None => {
                return Err(Error::DialogueDoesNotExist {
                    dialogue_index: index,
                    part_key: self.id.clone(),
                })
            }
        }
    }
}

#[derive(Debug)]
pub struct PartBuilder {
    id: String,
    quotes: Vec<Box<Quote>>,
}

impl PartBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            quotes: Vec::new(),
        }
    }
    pub fn add_quote(mut self, dialogue: impl Into<Box<Quote>>) -> Self {
        self.quotes.push(dialogue.into());
        self
    }
    pub fn build(self) -> Part {
        Part {
            id: self.id,
            quotes: self.quotes,
        }
    }
}

impl From<PartBuilder> for Part {
    fn from(value: PartBuilder) -> Self {
        Self {
            id: value.id,
            quotes: value.quotes,
        }
    }
}

impl Progressive for Part {
    type Output = Result<DialogueIndex>;
    fn next(&self, state: &mut State, choice_index: Option<usize>) -> Self::Output {
        if state.current_dialogue().is_none() {
            if !self.quotes.is_empty() {
                state.set_current_dialogue(Some(0));

                return Ok(DialogueIndex {
                    part_key: self.id().clone().into(),
                    dialogue_index: 0,
                });
            }
        } else {
            if let Some(dialogue_index) = state.current_dialogue() {
                let dialogue = self.quote(dialogue_index)?;
                let next_result = dialogue.next(state, choice_index)?;

                match next_result {
                    Some(next_part) => {
                        state.set_current_part(Some(next_part.clone().into()));
                        state.set_current_dialogue(Some(0));

                        return Ok(DialogueIndex {
                            part_key: next_part.clone().into(),
                            dialogue_index: 0,
                        });
                    }
                    None => {
                        let next_dialogue_index = dialogue_index + 1;
                        if self.quotes.get(next_dialogue_index).is_some() {
                            state.set_current_dialogue(Some(next_dialogue_index));

                            return Ok(DialogueIndex {
                                part_key: self.id().clone().into(),
                                dialogue_index: next_dialogue_index,
                            });
                        }
                    }
                }
            }
        }
        state.reset();
        Err(Error::EndOfStory)
    }
}
