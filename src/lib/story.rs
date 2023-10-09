use std::collections::HashMap;

use self::{
    context::Context,
    story_node::{dialogue::speaker::Speaker, StoryNode},
};

pub mod context;
pub mod story_link;
pub mod story_node;

pub struct Story {
    start: String,
    story_context: Context,
    current: Option<String>,
    speakers: HashMap<String, Speaker>,
    story_nodes: HashMap<String, StoryNode>,
}

impl Story {
    pub fn start(&self) -> &String {
        &self.start
    }
    pub fn story_context(&self) -> &Context {
        &self.story_context
    }
    pub fn current(&self) -> Option<&String> {
        match self.current {
            Some(ref current) => Some(current),
            None => None,
        }
    }
    pub fn speakers(&self) -> &HashMap<String, Speaker> {
        &self.speakers
    }
    pub fn story_nodes(&self) -> &HashMap<String, StoryNode> {
        &self.story_nodes
    }
}
pub struct StoryBuilder {
    pub start: String,
    pub story_context: Context,
    current: Option<String>,
    pub speakers: HashMap<String, Speaker>,
    pub story_nodes: HashMap<String, StoryNode>,
}

impl StoryBuilder {
    pub fn new(node_key: String) -> Self {
        Self {
            start: node_key,
            story_context: Context::new(),
            current: None,
            speakers: HashMap::new(),
            story_nodes: HashMap::new(),
        }
    }

    pub fn set_current(&mut self, node_key: Option<String>) {
        self.current = node_key;
    }
}
