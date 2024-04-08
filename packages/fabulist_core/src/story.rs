use std::collections::HashMap;

use crate::{
    error::{Error, Result},
    state::{DialogueIndex, State},
};

use self::{character::Character, part::Part, traits::Progressive};

pub mod actions;
pub mod character;
pub mod choice;
pub mod context;
pub mod dialogue;
pub mod part;
pub mod traits;

pub struct Story {
    start: Option<String>,
    characters: HashMap<String, Character>,
    parts: HashMap<String, Part>,
}

impl Story {
    pub fn start(&self) -> Option<String> {
        match self.start.as_ref() {
            Some(start) => Some(start.clone()),
            None => None,
        }
    }
    pub fn characters(&self) -> &HashMap<String, Character> {
        &self.characters
    }
    pub fn parts(&self) -> &HashMap<String, Part> {
        &self.parts
    }
    pub fn part(&self, key: &String) -> Result<&Part> {
        match self.parts.get(key) {
            Some(part) => Ok(part),
            None => Err(Error::PartDoesNotExist {
                part_key: key.clone(),
            }),
        }
    }
    pub fn mut_part(&mut self, key: &String) -> Result<&mut Part> {
        match self.parts.get_mut(key) {
            Some(part) => Ok(part),
            None => Err(Error::PartDoesNotExist {
                part_key: key.clone(),
            }),
        }
    }
}

pub struct StoryBuilder {
    start: Option<String>,
    characters: HashMap<String, Character>,
    parts: HashMap<String, Part>,
}

impl StoryBuilder {
    pub fn new() -> Self {
        Self {
            start: None,
            characters: HashMap::new(),
            parts: HashMap::new(),
        }
    }
    pub fn set_start(mut self, part_key: impl Into<String>) -> Self {
        self.start = Some(part_key.into());
        self
    }
    pub fn add_part(mut self, part: Part) -> Self {
        self.parts.insert(part.id().clone(), part);
        self
    }
    pub fn add_character(mut self, character: Character) -> Self {
        self.characters.insert(character.id.clone(), character);
        self
    }
    pub fn build(self) -> Story {
        Story {
            start: self.start,
            characters: self.characters,
            parts: self.parts,
        }
    }
}

impl Progressive for Story {
    type Output = Result<DialogueIndex>;
    fn next(&self, state: &mut State, choice_index: Option<usize>) -> Self::Output {
        if let Some(part_key) = state.current_part() {
            let part = self.part(part_key)?;
            return part.next(state, choice_index);
        }
        state.reset();
        Err(Error::EndOfStory)
    }
}
