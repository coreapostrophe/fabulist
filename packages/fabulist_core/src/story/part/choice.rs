use std::fmt::Debug;

use crate::{
    error::EngineResult,
    state::State,
    story::{reference::ListKey, Progressive},
};

use super::actions::{ChangeContext, ChangeContextClosure, QueryNext, QueryNextClosure};

pub struct Choice {
    text: String,
    response: Option<String>,
    query_next: Option<QueryNextClosure>,
    change_context: Option<ChangeContextClosure>,
}

impl Debug for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Choice")
            .field("text", &self.text)
            .field("response", &self.response)
            .field("query_next", &self.query_next.is_some())
            .field("change_context", &self.change_context.is_some())
            .finish()
    }
}

impl Choice {
    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    pub fn response(&self) -> Option<&String> {
        self.response.as_ref()
    }

    pub fn set_response(&mut self, response: Option<String>) {
        self.response = response;
    }
}

impl QueryNext for Choice {
    fn query_next(&self) -> Option<&QueryNextClosure> {
        self.query_next.as_ref()
    }

    fn set_query_next(&mut self, closure: QueryNextClosure) {
        self.query_next = Some(closure);
    }
}

impl ChangeContext for Choice {
    fn change_context(&self) -> Option<&ChangeContextClosure> {
        self.change_context.as_ref()
    }

    fn set_change_context(&mut self, closure: ChangeContextClosure) {
        self.change_context = Some(closure);
    }
}

pub struct ChoiceBuilder {
    text: String,
    response: Option<String>,
    query_next: Option<QueryNextClosure>,
    change_context: Option<ChangeContextClosure>,
}

impl ChoiceBuilder {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            response: None,
            query_next: None,
            change_context: None,
        }
    }

    pub fn set_response(mut self, response: String) -> Self {
        self.response = Some(response);
        self
    }

    pub fn set_query_next(mut self, closure: QueryNextClosure) -> Self {
        self.query_next = Some(Box::new(closure));
        self
    }

    pub fn set_change_context(mut self, closure: ChangeContextClosure) -> Self {
        self.change_context = Some(Box::new(closure));
        self
    }

    pub fn build(self) -> Choice {
        Choice {
            text: self.text,
            response: self.response,
            query_next: self.query_next,
            change_context: self.change_context,
        }
    }
}

impl From<ChoiceBuilder> for Choice {
    fn from(value: ChoiceBuilder) -> Self {
        Self {
            text: value.text,
            response: value.response,
            query_next: value.query_next,
            change_context: value.change_context,
        }
    }
}

impl Progressive for Choice {
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
