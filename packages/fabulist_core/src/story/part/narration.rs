use fabulist_derive::ElementInternal;

use crate::{engine::Progressive, error::EngineResult, state::State, story::reference::ListKey};

use super::{
    actions::{ChangeContext, ChangeContextClosure, QueryNext, QueryNextClosure},
    PartElement,
};

#[derive(ElementInternal, Debug)]
pub struct Narration {
    text: String,
    query_next: Option<QueryNextClosure>,
    change_context: Option<ChangeContextClosure>,
}

impl Narration {
    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }
}

impl QueryNext for Narration {
    fn query_next(&self) -> Option<&QueryNextClosure> {
        self.query_next.as_ref()
    }

    fn set_query_next(&mut self, closure: QueryNextClosure) {
        self.query_next = Some(closure);
    }
}

impl ChangeContext for Narration {
    fn change_context(&self) -> Option<&ChangeContextClosure> {
        self.change_context.as_ref()
    }

    fn set_change_context(&mut self, closure: ChangeContextClosure) {
        self.change_context = Some(closure);
    }
}

pub struct NarrationBuilder {
    text: String,
    query_next: Option<QueryNextClosure>,
    change_context: Option<ChangeContextClosure>,
}

impl NarrationBuilder {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
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

    pub fn build(self) -> Narration {
        Narration {
            text: self.text,
            query_next: self.query_next,
            change_context: self.change_context,
        }
    }
}

impl From<NarrationBuilder> for Narration {
    fn from(value: NarrationBuilder) -> Self {
        Self {
            text: value.text,
            query_next: value.query_next,
            change_context: value.change_context,
        }
    }
}

impl From<NarrationBuilder> for Box<PartElement> {
    fn from(value: NarrationBuilder) -> Self {
        Box::new(Narration {
            text: value.text,
            query_next: value.query_next,
            change_context: value.change_context,
        })
    }
}

impl Progressive for Narration {
    type Output = EngineResult<Option<ListKey<String>>>;
    fn next(&self, state: &mut State, _choice_index: Option<usize>) -> Self::Output {
        if let Some(change_context_closure) = self.change_context {
            change_context_closure(state.mut_context());
        }
        let next_part_key = self
            .query_next
            .map(|next_closure| next_closure(state.context()));
        Ok(next_part_key)
    }
}
