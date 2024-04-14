use crate::{
    error::Result,
    state::State,
    story::{
        character::Character,
        resource::{Inset, InterpInset},
        Progressive,
    },
};

use super::{
    actions::{ChangeContextClosure, QueryNextClosure},
    Element, PartElement,
};

#[derive(Debug)]
pub struct Dialogue {
    text: String,
    character: Inset<Character>,
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
    pub fn character(&self) -> &Inset<Character> {
        &self.character
    }
    pub fn set_character(&mut self, id: impl Into<String>) {
        self.character.set_id(id);
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
    character: Inset<Character>,
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
            character: Inset::new(layout.character.to_string()),
            query_next: None,
            change_context: None,
        }
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
            query_next: value.query_next,
            change_context: value.change_context,
        }
    }
}

impl InterpInset for Dialogue {
    fn interp_inset(&mut self, resources: &mut crate::story::resource::Resources) {
        let resource = resources.get::<Character>(self.character.id());
        self.character.set_value(resource.clone());
    }
}

impl From<DialogueBuilder> for Box<PartElement> {
    fn from(value: DialogueBuilder) -> Self {
        Box::new(Dialogue {
            text: value.text,
            character: value.character,
            query_next: value.query_next,
            change_context: value.change_context,
        })
    }
}

impl Element for Dialogue {}

impl Progressive for Dialogue {
    type Output = Result<Option<String>>;
    fn next(&self, state: &mut State, _choice_index: Option<usize>) -> Self::Output {
        match self.change_context {
            Some(change_context_closure) => {
                change_context_closure(state.mut_context());
            }
            None => (),
        }
        let next_part_key = match self.query_next {
            Some(next_closure) => Some(next_closure(state.context())),
            None => None,
        };
        Ok(next_part_key)
    }
}
