use std::collections::HashMap;

use crate::story::{
    context::Context,
    reference::{DialogueIndex, ListKey},
};

#[derive(Debug)]
pub struct State {
    context: Context,
    current_part: Option<ListKey<String>>,
    current_element: Option<usize>,
    decisions: HashMap<DialogueIndex, usize>,
}

impl State {
    pub fn new() -> Self {
        Self {
            current_part: None,
            current_element: None,
            context: Context::new(),
            decisions: HashMap::new(),
        }
    }
    pub fn decisions(&self) -> &HashMap<DialogueIndex, usize> {
        &self.decisions
    }
    pub fn mut_decisions(&mut self) -> &mut HashMap<DialogueIndex, usize> {
        &mut self.decisions
    }
    pub fn context(&self) -> &Context {
        &self.context
    }
    pub fn mut_context(&mut self) -> &mut Context {
        &mut self.context
    }
    pub fn current_part(&self) -> Option<&ListKey<String>> {
        self.current_part.as_ref()
    }
    pub fn set_current_part(&mut self, part_key: Option<ListKey<String>>) {
        self.current_part = part_key;
    }
    pub fn current_element(&self) -> Option<usize> {
        match self.current_element {
            Some(element_index) => Some(element_index),
            None => None,
        }
    }
    pub fn set_current_element(&mut self, element_index: Option<usize>) {
        self.current_element = element_index;
    }
    pub fn reset(&mut self) {
        self.current_part = None;
        self.current_element = None;
    }
}
