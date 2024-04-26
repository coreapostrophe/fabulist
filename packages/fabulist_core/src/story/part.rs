use crate::{
    error::{Error, Result},
    state::State,
};

use super::{reference::ListKey, resource::InterpInset, DialogueIndex, Progressive};

#[cfg(feature = "actions")]
pub mod actions;
#[cfg(feature = "selection")]
pub mod choice;
#[cfg(feature = "dialogue")]
pub mod dialogue;
#[cfg(feature = "narration")]
pub mod narration;
#[cfg(feature = "selection")]
pub mod selection;

pub trait Element: Progressive + InterpInset {}

pub type PartElement = dyn Element<Output = Result<Option<ListKey<String>>>>;

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
            Some(element) => Ok(element),
            None => {
                Err(Error::ElementDoesNotExist {
                    dialogue_index: index,
                    part_key: self.id.clone(),
                })
            }
        }
    }
    pub fn mut_element(&mut self, index: usize) -> Result<&mut Box<PartElement>> {
        match self.elements.get_mut(index) {
            Some(element) => Ok(element),
            None => {
                Err(Error::ElementDoesNotExist {
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

impl InterpInset for Part {
    fn interp_inset(&mut self, resource: &mut super::resource::Resources) {
        self.elements
            .iter_mut()
            .for_each(|element| element.interp_inset(resource));
    }
}

impl Progressive for Part {
    type Output = Result<DialogueIndex>;
    fn next(&self, state: &mut State, choice_index: Option<usize>) -> Self::Output {
        if state.current_element().is_none() {
            if !self.elements.is_empty() {
                state.set_current_element(Some(0));

                return Ok(DialogueIndex {
                    part_key: self.id().clone().into(),
                    dialogue_index: 0,
                });
            }
        } else if let Some(dialogue_index) = state.current_element() {
            let dialogue = self.element(dialogue_index)?;
            let next_result = dialogue.next(state, choice_index)?;

            match next_result {
                Some(next_part) => {
                    state.set_current_part(Some(next_part.clone()));
                    state.set_current_element(Some(0));

                    return Ok(DialogueIndex {
                        part_key: next_part.clone(),
                        dialogue_index: 0,
                    });
                }
                None => {
                    let next_dialogue_index = dialogue_index + 1;
                    if self.elements.get(next_dialogue_index).is_some() {
                        state.set_current_element(Some(next_dialogue_index));

                        return Ok(DialogueIndex {
                            part_key: self.id().clone().into(),
                            dialogue_index: next_dialogue_index,
                        });
                    }
                }
            }
        }
        state.reset();
        Err(Error::EndOfStory)
    }
}
