use std::collections::HashMap;

use crate::story::{
    context::{Context, Mappable},
    reference::{DialogueIndex, ListKey},
};

#[derive(Debug)]
pub struct State {
    context: Box<dyn Mappable>,
    current_part: Option<ListKey<String>>,
    current_element: Option<usize>,
    decisions: HashMap<DialogueIndex, usize>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            context: Box::new(Context::new()),
            current_part: None,
            current_element: None,
            decisions: HashMap::new(),
        }
    }
}

pub type ContextState = Box<dyn Mappable>;

pub struct StateOptions {
    pub context: Option<ContextState>,
    pub current_part: Option<ListKey<String>>,
    pub current_element: Option<usize>,
    pub decisions: Option<HashMap<DialogueIndex, usize>>,
}

impl State {
    pub fn new(options: Option<StateOptions>) -> Self {
        if let Some(options) = options {
            Self {
                current_part: options.current_part,
                current_element: options.current_element,
                context: options.context.unwrap_or_else(|| Box::new(Context::new())),
                decisions: options.decisions.unwrap_or_else(HashMap::new),
            }
        } else {
            Self::default()
        }
    }

    pub fn decisions(&self) -> &HashMap<DialogueIndex, usize> {
        &self.decisions
    }

    pub fn mut_decisions(&mut self) -> &mut HashMap<DialogueIndex, usize> {
        &mut self.decisions
    }

    pub fn context(&self) -> &ContextState {
        &self.context
    }

    pub fn mut_context(&mut self) -> &mut ContextState {
        &mut self.context
    }

    pub fn current_part(&self) -> Option<&ListKey<String>> {
        self.current_part.as_ref()
    }

    pub fn set_current_part(&mut self, part_key: Option<ListKey<String>>) {
        self.current_part = part_key;
    }

    pub fn current_element(&self) -> Option<usize> {
        self.current_element
    }

    pub fn set_current_element(&mut self, element_index: Option<usize>) {
        self.current_element = element_index;
    }

    pub fn reset(&mut self) {
        self.current_part = None;
        self.current_element = None;
    }
}
