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

pub type PartElement = ProgressiveElement<Result<Option<String>>>;

#[derive(Debug)]
pub struct Part {
    id: String,
    elements: Vec<Box<PartElement>>,
}

impl Part {
    pub fn id(&self) -> &String {
        &self.id
    }
    pub fn elements(&self) -> &Vec<Box<PartElement>> {
        &self.elements
    }
    pub fn mut_elements(&mut self) -> &mut Vec<Box<PartElement>> {
        &mut self.elements
    }
    pub fn element(&self, index: usize) -> Result<&Box<PartElement>> {
        match self.elements.get(index) {
            Some(dialogue) => Ok(dialogue),
            None => {
                return Err(Error::DialogueDoesNotExist {
                    dialogue_index: index,
                    part_key: self.id.clone(),
                })
            }
        }
    }
    pub fn mut_element(&mut self, index: usize) -> Result<&mut Box<PartElement>> {
        match self.elements.get_mut(index) {
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
    quotes: Vec<Box<PartElement>>,
}

impl PartBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            quotes: Vec::new(),
        }
    }
    pub fn add_element(mut self, dialogue: impl Into<Box<PartElement>>) -> Self {
        self.quotes.push(dialogue.into());
        self
    }
    pub fn build(self) -> Part {
        Part {
            id: self.id,
            elements: self.quotes,
        }
    }
}

impl From<PartBuilder> for Part {
    fn from(value: PartBuilder) -> Self {
        Self {
            id: value.id,
            elements: value.quotes,
        }
    }
}

impl Progressive for Part {
    type Output = Result<DialogueIndex>;
    fn next(&self, state: &mut State, choice_index: Option<usize>) -> Self::Output {
        if state.current_dialogue().is_none() {
            if !self.elements.is_empty() {
                state.set_current_dialogue(Some(0));

                return Ok(DialogueIndex {
                    part_key: self.id().clone().into(),
                    dialogue_index: 0,
                });
            }
        } else {
            if let Some(dialogue_index) = state.current_dialogue() {
                let dialogue = self.element(dialogue_index)?;
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
                        if self.elements.get(next_dialogue_index).is_some() {
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
