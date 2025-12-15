use fabulist_derive::ElementInternal;

use crate::{
    error::EngineResult,
    state::State,
    story::{character::Character, reference::ListKey, resource::Inset, Progressive},
};

use super::{
    actions::{ChangeContext, ChangeContextClosure, QueryNext, QueryNextClosure},
    PartElement,
};

#[derive(ElementInternal)]
pub struct Dialogue {
    text: String,
    character: Inset<Character>,
    query_next: Option<QueryNextClosure>,
    change_context: Option<ChangeContextClosure>,
}

impl std::fmt::Debug for Dialogue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dialogue")
            .field("text", &self.text)
            .field("character", &self.character)
            .field("query_next", &self.query_next.is_some())
            .field("change_context", &self.change_context.is_some())
            .finish()
    }
}

impl Dialogue {
    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }
}

impl QueryNext for Dialogue {
    fn query_next(&self) -> Option<&QueryNextClosure> {
        self.query_next.as_ref()
    }

    fn set_query_next(&mut self, closure: QueryNextClosure) {
        self.query_next = Some(closure);
    }
}

impl ChangeContext for Dialogue {
    fn change_context(&self) -> Option<&ChangeContextClosure> {
        self.change_context.as_ref()
    }

    fn set_change_context(&mut self, closure: ChangeContextClosure) {
        self.change_context = Some(closure);
    }
}

pub struct DialogueBuilder {
    text: String,
    character: Inset<Character>,
    query_next: Option<QueryNextClosure>,
    change_context: Option<ChangeContextClosure>,
}

impl std::fmt::Debug for DialogueBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DialogueBuilder")
            .field("text", &self.text)
            .field("character", &self.character)
            .field("query_next", &self.query_next.is_some())
            .field("change_context", &self.change_context.is_some())
            .finish()
    }
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

impl Progressive for Dialogue {
    type Output = EngineResult<Option<ListKey<String>>>;
    fn next(&self, state: &mut State, _choice_index: Option<usize>) -> Self::Output {
        if let Some(change_context_closure) = self.change_context.as_ref() {
            change_context_closure(state.mut_context().as_mut())?;
        }

        let next_part_key = self
            .query_next
            .as_ref()
            .map(|next_closure| next_closure(state.context().as_ref()))
            .transpose()?;

        Ok(next_part_key)
    }
}
