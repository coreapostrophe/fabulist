use std::collections::HashMap;

use crate::{
    error::{Error, Result},
    state::State,
};

use self::{character::Character, dialogue::Dialogue, part::Part, traits::Progressive};

pub mod actions;
pub mod character;
pub mod choice;
pub mod context;
pub mod dialogue;
pub mod part;
pub mod traits;

#[derive(Debug)]
pub struct DialogueIndex {
    pub part_key: String,
    pub dialogue_index: usize,
}

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
    pub fn set_start(&mut self, start: Option<String>) {
        self.start = start;
    }
    pub fn characters(&self) -> &HashMap<String, Character> {
        &self.characters
    }
    pub fn mut_characters(&mut self) -> &mut HashMap<String, Character> {
        &mut self.characters
    }
    pub fn parts(&self) -> &HashMap<String, Part> {
        &self.parts
    }
    pub fn mut_parts(&mut self) -> &mut HashMap<String, Part> {
        &mut self.parts
    }
    pub fn character(&self, key: &str) -> Result<&Character> {
        match self.characters.get(key) {
            Some(character) => Ok(character),
            None => Err(Error::CharacterDoesNotExist { key: key.into() }),
        }
    }
    pub fn mut_character(&mut self, key: &str) -> Result<&mut Character> {
        match self.characters.get_mut(key) {
            Some(character) => Ok(character),
            None => Err(Error::CharacterDoesNotExist { key: key.into() }),
        }
    }
    pub fn part(&self, key: &str) -> Result<&Part> {
        match self.parts.get(key) {
            Some(part) => Ok(part),
            None => Err(Error::PartDoesNotExist { key: key.into() }),
        }
    }
    pub fn mut_part(&mut self, key: &str) -> Result<&mut Part> {
        match self.parts.get_mut(key) {
            Some(part) => Ok(part),
            None => Err(Error::PartDoesNotExist { key: key.into() }),
        }
    }
    pub fn dialogue(&self, index: DialogueIndex) -> Result<&Dialogue> {
        let part_key = &index.part_key;
        let part = self.part(part_key)?;
        let dialogue_index = &index.dialogue_index;
        part.dialogue(*dialogue_index)
    }
    pub fn mut_dialogue(&mut self, index: DialogueIndex) -> Result<&mut Dialogue> {
        let part_key = &index.part_key;
        let part = self.mut_part(part_key)?;
        let dialogue_index = &index.dialogue_index;
        part.mut_dialogue(*dialogue_index)
    }
}

#[derive(Debug)]
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
    pub fn add_part(mut self, part: impl Into<Part>) -> Self {
        let part = part.into();
        self.parts.insert(part.id().clone(), part);
        self
    }
    pub fn add_character(mut self, character: impl Into<Character>) -> Self {
        let character = character.into();
        self.characters.insert(character.id().clone(), character);
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

impl From<StoryBuilder> for Story {
    fn from(value: StoryBuilder) -> Self {
        Self {
            start: value.start,
            characters: value.characters,
            parts: value.parts,
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
