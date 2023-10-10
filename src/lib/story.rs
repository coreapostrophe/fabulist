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

    pub fn set_current(&mut self, node_key: String) {
        self.current = Some(node_key);
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
    pub fn new(start_node_key: &str) -> Self {
        Self {
            start: start_node_key.to_string(),
            story_context: Context::new(),
            current: None,
            speakers: HashMap::new(),
            story_nodes: HashMap::new(),
        }
    }
    pub fn set_current(mut self, node_key: &str) -> Self {
        self.current = Some(node_key.to_string());
        self
    }
    pub fn add_node(mut self, key: &str, node: StoryNode) -> Self {
        self.story_nodes.insert(key.to_string(), node);
        self
    }
    pub fn add_speaker(mut self, key: &str, speaker: Speaker) -> Self {
        self.speakers.insert(key.to_string(), speaker);
        self
    }
    pub fn build(self) -> Story {
        Story {
            start: self.start,
            current: self.current,
            speakers: self.speakers,
            story_nodes: self.story_nodes,
            story_context: self.story_context
        }
    }
}

#[cfg(test)]
mod story_tests {
    use super::{*, story_node::dialogue::DialogueBuilder};


    #[test]
    fn matches_use_spec() {
        let story = StoryBuilder::new("mock-start-node")
            .set_current("mock-current-node")
            .add_speaker("speaker-1", Speaker::new("Speaker 1"))
            .add_node("story-node-1", StoryNode::Dialogue(DialogueBuilder::new("speaker-1").build()))
            .build();
        assert_eq!(story.start().as_str(), "mock-start-node");
        assert_eq!(story.current().unwrap().as_str(), "mock-current-node");

        assert_eq!(story.speakers().len(), 1);
        assert_eq!(story.speakers().get("speaker-1").unwrap().name().as_str(), "Speaker 1");

        assert_eq!(story.story_nodes().len(), 1);

        let _ = match story.story_nodes().get("story-node-1").unwrap() {
            StoryNode::Dialogue(dialogue) => {
                assert_eq!(dialogue.speaker().as_str(), "speaker-1");
            },
            StoryNode::Part(_) => panic!("should be a dialogue")
        };
    }
}