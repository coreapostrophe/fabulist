use crate::{engine::Progressive, error::Result, state::State, story::resource::InterpInset};

use super::{
    actions::{ChangeContext, ChangeContextClosure, QueryNext, QueryNextClosure},
    Element, PartElement,
};

#[derive(Debug)]
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

impl InterpInset for Narration {
    fn interp_inset(&mut self, _resources: &mut crate::story::resource::Resources) {}
}

impl Element for Narration {}

impl Progressive for Narration {
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