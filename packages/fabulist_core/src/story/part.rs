use crate::{
    error::{Error, Result},
    state::State,
};

use super::{dialogue::Dialogue, traits::Progressive, DialogueIndex};

#[derive(Debug)]
pub struct Part {
    id: String,
    dialogues: Vec<Dialogue>,
}

impl Part {
    pub fn id(&self) -> &String {
        &self.id
    }
    pub fn dialogues(&self) -> &Vec<Dialogue> {
        &self.dialogues
    }
    pub fn mut_dialogues(&mut self) -> &mut Vec<Dialogue> {
        &mut self.dialogues
    }
    pub fn dialogue(&self, index: usize) -> Result<&Dialogue> {
        match self.dialogues.get(index) {
            Some(dialogue) => Ok(dialogue),
            None => {
                return Err(Error::DialogueDoesNotExist {
                    dialogue_index: index,
                    part_key: self.id.clone(),
                })
            }
        }
    }
    pub fn mut_dialogue(&mut self, index: usize) -> Result<&mut Dialogue> {
        match self.dialogues.get_mut(index) {
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
    dialogues: Vec<Dialogue>,
}

impl PartBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            dialogues: Vec::new(),
        }
    }
    pub fn add_dialogue(mut self, dialogue: impl Into<Dialogue>) -> Self {
        self.dialogues.push(dialogue.into());
        self
    }
    pub fn build(self) -> Part {
        Part {
            id: self.id,
            dialogues: self.dialogues,
        }
    }
}

impl From<PartBuilder> for Part {
    fn from(value: PartBuilder) -> Self {
        Self {
            id: value.id,
            dialogues: value.dialogues,
        }
    }
}

impl Progressive for Part {
    type Output = Result<DialogueIndex>;
    fn next(&self, state: &mut State, choice_index: Option<usize>) -> Self::Output {
        if state.current_dialogue().is_none() {
            if !self.dialogues.is_empty() {
                state.set_current_dialogue(Some(0));

                return Ok(DialogueIndex {
                    part_key: self.id().clone().into(),
                    dialogue_index: 0,
                });
            }
        } else {
            if let Some(dialogue_index) = state.current_dialogue() {
                let dialogue = self.dialogue(dialogue_index)?;
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
                        if self.dialogues.get(next_dialogue_index).is_some() {
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
