use std::{collections::HashMap, fmt::Debug};

use crate::{
    engine::Progressive,
    error::{EngineError, EngineResult},
    state::State,
};

#[cfg(feature = "parsing")]
use crate::error::{ParsingError, ParsingResult};

use self::{
    part::{Part, PartElement},
    reference::{DialogueIndex, ListKey},
    resource::{InterpInset, Keyed, Resources},
};

pub mod character;
pub mod context;
pub mod part;
pub mod reference;
pub mod resource;

#[cfg(feature = "parsing")]
pub mod lang_converters;

#[derive(Debug)]
pub struct Story {
    start: Option<ListKey<String>>,
    parts: HashMap<ListKey<String>, Part>,
    resources: Resources,
}

impl Default for Story {
    fn default() -> Self {
        Self::new()
    }
}

impl Story {
    pub fn new() -> Self {
        Self {
            start: None,
            parts: HashMap::new(),
            resources: Resources::new(),
        }
    }

    pub fn start(&self) -> Option<&ListKey<String>> {
        match self.start.as_ref() {
            Some(start) => Some(start),
            None => None,
        }
    }

    pub fn set_start(&mut self, start: Option<ListKey<String>>) {
        self.start = start;
    }

    pub fn parts(&self) -> &HashMap<ListKey<String>, Part> {
        &self.parts
    }

    pub fn add_part_module<const N: usize>(
        &mut self,
        module_key: [&str; N],
        part: impl Into<Part>,
    ) {
        let part = part.into();
        let part_key = [&module_key[..], &[part.id()]].concat();
        self.parts.insert(part_key.into(), part);
    }

    pub fn add_part(&mut self, part: impl Into<Part>) {
        self.add_part_module([], part);
    }

    pub fn mut_parts(&mut self) -> &mut HashMap<ListKey<String>, Part> {
        &mut self.parts
    }

    pub fn part(&self, key: &ListKey<String>) -> EngineResult<&Part> {
        match self.parts.get(key) {
            Some(part) => Ok(part),
            None => Err(EngineError::PartDoesNotExist {
                key: key.to_owned(),
            }),
        }
    }

    pub fn mut_part(&mut self, key: &ListKey<String>) -> EngineResult<&mut Part> {
        match self.parts.get_mut(key) {
            Some(part) => Ok(part),
            None => Err(EngineError::PartDoesNotExist {
                key: key.to_owned(),
            }),
        }
    }

    pub fn resources(&self) -> &Resources {
        &self.resources
    }

    pub fn mut_resources(&mut self) -> &mut Resources {
        &mut self.resources
    }

    pub fn element(&self, index: DialogueIndex) -> EngineResult<&PartElement> {
        let part_key = &index.part_key;
        let part = self.part(part_key)?;
        let dialogue_index = &index.dialogue_index;
        part.element(*dialogue_index)
    }

    pub fn mut_element(&mut self, index: DialogueIndex) -> EngineResult<&mut Box<PartElement>> {
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
    resources: Resources,
}

impl Default for StoryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl StoryBuilder {
    pub fn new() -> Self {
        Self {
            start: None,
            parts: HashMap::new(),
            resources: Resources::new(),
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

    pub fn add_resource<T>(mut self, resource: impl Into<T>) -> Self
    where
        T: Keyed + Clone + 'static,
    {
        self.resources.insert::<T>(resource.into());
        self
    }

    pub fn add_res_collection<T, const N: usize>(mut self, collection: [T; N]) -> Self
    where
        T: Keyed + Clone + 'static,
    {
        self.resources.insert_collection::<T, N>(collection);
        self
    }

    #[cfg(feature = "parsing")]
    pub fn parse(source: impl Into<String>) -> ParsingResult<Self> {
        let _story_ast = fabulist_lang::parser::FabulistParser::parse(source.into())
            .map_err(|err| ParsingError::from(Box::new(*err)))?;

        todo!()
    }

    pub fn build(mut self) -> Story {
        self.parts
            .iter_mut()
            .for_each(|(_, part)| part.interp_inset(&mut self.resources));

        Story {
            start: self.start,
            parts: self.parts,
            resources: self.resources,
        }
    }
}

impl From<StoryBuilder> for Story {
    fn from(value: StoryBuilder) -> Self {
        Self {
            start: value.start,
            parts: value.parts,
            resources: value.resources,
        }
    }
}

impl Progressive for Story {
    type Output = EngineResult<DialogueIndex>;
    fn next(&self, state: &mut State, choice_index: Option<usize>) -> Self::Output {
        if let Some(part_key) = state.current_part() {
            let part = self.part(part_key)?;
            return part.next(state, choice_index);
        }
        state.reset();
        Err(EngineError::EndOfStory)
    }
}
