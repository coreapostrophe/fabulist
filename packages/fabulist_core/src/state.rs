use std::collections::HashMap;

use crate::story::{context::Context, DialogueIndex};

#[derive(Debug)]
pub struct State {
    context: Context,
    current_part: Option<String>,
    current_dialogue: Option<usize>,
    decisions: HashMap<DialogueIndex, usize>,
}

impl State {
    pub fn new() -> Self {
        Self {
            current_part: None,
            current_dialogue: None,
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
    pub fn current_part(&self) -> Option<&String> {
        self.current_part.as_ref()
    }
    pub fn set_current_part(&mut self, part_key: Option<String>) {
        self.current_part = part_key;
    }
    pub fn current_dialogue(&self) -> Option<usize> {
        match self.current_dialogue {
            Some(dialogue_index) => Some(dialogue_index),
            None => None,
        }
    }
    pub fn set_current_dialogue(&mut self, dialogue_index: Option<usize>) {
        self.current_dialogue = dialogue_index;
    }
    pub fn reset(&mut self) {
        self.current_part = None;
        self.current_dialogue = None;
    }
}
