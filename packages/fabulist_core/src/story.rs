use std::collections::HashMap;

use crate::{
    error::{Error, Result},
    state::State,
};

use self::{
    character::Character,
    part::{Part, PartElement},
    reference::{DialogueIndex, ListKey},
    traits::Progressive,
};

pub mod character;
pub mod context;
pub mod part;
pub mod reference;
pub mod traits;

#[derive(Debug)]
pub struct Story {
    start: Option<ListKey<String>>,
    parts: HashMap<ListKey<String>, Part>,
    characters: HashMap<ListKey<String>, Character>,
}

impl Story {
    pub fn start(&self) -> Option<&ListKey<String>> {
        match self.start.as_ref() {
            Some(start) => Some(start),
            None => None,
        }
    }
    pub fn set_start(&mut self, start: Option<ListKey<String>>) {
        self.start = start;
    }
    pub fn characters(&self) -> &HashMap<ListKey<String>, Character> {
        &self.characters
    }
    pub fn mut_characters(&mut self) -> &mut HashMap<ListKey<String>, Character> {
        &mut self.characters
    }
    pub fn parts(&self) -> &HashMap<ListKey<String>, Part> {
        &self.parts
    }
    pub fn mut_parts(&mut self) -> &mut HashMap<ListKey<String>, Part> {
        &mut self.parts
    }
    pub fn character(&self, key: &str) -> Result<&Character> {
        match self.characters.get(&key.into()) {
            Some(character) => Ok(character),
            None => Err(Error::CharacterDoesNotExist {
                key: key.to_owned(),
            }),
        }
    }
    pub fn mut_character(&mut self, key: &str) -> Result<&mut Character> {
        match self.characters.get_mut(&key.into()) {
            Some(character) => Ok(character),
            None => Err(Error::CharacterDoesNotExist {
                key: key.to_owned(),
            }),
        }
    }
    pub fn part(&self, key: &ListKey<String>) -> Result<&Part> {
        match self.parts.get(&key) {
            Some(part) => Ok(part),
            None => Err(Error::PartDoesNotExist {
                key: key.to_owned(),
            }),
        }
    }
    pub fn mut_part(&mut self, key: &ListKey<String>) -> Result<&mut Part> {
        match self.parts.get_mut(key) {
            Some(part) => Ok(part),
            None => Err(Error::PartDoesNotExist {
                key: key.to_owned(),
            }),
        }
    }
    pub fn dialogue(&self, index: DialogueIndex) -> Result<&Box<PartElement>> {
        let part_key = &index.part_key;
        let part = self.part(part_key)?;
        let dialogue_index = &index.dialogue_index;
        part.element(*dialogue_index)
    }
    pub fn mut_dialogue(&mut self, index: DialogueIndex) -> Result<&mut Box<PartElement>> {
        let part_key = &index.part_key;
        let part = self.mut_part(part_key)?;
        let dialogue_index = &index.dialogue_index;
        part.mut_element(*dialogue_index)
    }
}

#[derive(Debug)]
pub struct StoryBuilder {
    start: Option<ListKey<String>>,
    parts: HashMap<ListKey<String>, Part>,
    characters: HashMap<ListKey<String>, Character>,
}

impl StoryBuilder {
    pub fn new() -> Self {
        Self {
            start: None,
            characters: HashMap::new(),
            parts: HashMap::new(),
        }
    }
    pub fn set_start(mut self, part_key: impl Into<ListKey<String>>) -> Self {
        self.start = Some(part_key.into());
        self
    }
    pub fn add_part_module<const N: usize>(
        mut self,
        module_key: [&str; N],
        part: impl Into<Part>,
    ) -> Self {
        let part = part.into();
        let part_key = [&module_key[..], &[part.id()]].concat();
        self.parts.insert(part_key.into(), part);
        self
    }
    pub fn add_part(self, part: impl Into<Part>) -> Self {
        self.add_part_module([], part)
    }
    pub fn add_character_module<const N: usize>(
        mut self,
        module_key: [&str; N],
        character: impl Into<Character>,
    ) -> Self {
        let character = character.into();
        let character_key = [&module_key[..], &[character.id()]].concat();
        self.characters.insert(character_key.into(), character);
        self
    }
    pub fn add_character(self, character: impl Into<Character>) -> Self {
        self.add_character_module([], character)
    }
    pub fn build(self) -> Story {
        Story {
            start: self.start,
            parts: self.parts,
            characters: self.characters,
        }
    }
}

impl From<StoryBuilder> for Story {
    fn from(value: StoryBuilder) -> Self {
        Self {
            start: value.start,
            parts: value.parts,
            characters: value.characters,
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
