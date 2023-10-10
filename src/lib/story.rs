use std::collections::HashMap;

use self::story_node::{dialogue::speaker::Speaker, StoryNode};

pub mod context;
pub mod story_link;
pub mod story_node;

pub struct Story {
    start: String,
    speakers: HashMap<String, Speaker>,
    story_nodes: HashMap<String, StoryNode>,
}

impl Story {
    pub fn start(&self) -> &String {
        &self.start
    }
    pub fn speakers(&self) -> &HashMap<String, Speaker> {
        &self.speakers
    }
    pub fn story_nodes(&self) -> &HashMap<String, StoryNode> {
        &self.story_nodes
    }
    pub fn mut_story_nodes(&mut self) -> &mut HashMap<String, StoryNode> {
        &mut self.story_nodes
    }
}

pub struct StoryBuilder {
    start: String,
    speakers: HashMap<String, Speaker>,
    story_nodes: HashMap<String, StoryNode>,
}

impl StoryBuilder {
    pub fn new(start_node_key: &str) -> Self {
        Self {
            start: start_node_key.to_string(),
            speakers: HashMap::new(),
            story_nodes: HashMap::new(),
        }
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
            speakers: self.speakers,
            story_nodes: self.story_nodes,
        }
    }
}

#[cfg(test)]
mod story_tests {
    use super::{*, story_node::dialogue::DialogueBuilder};

    #[test]
    fn matches_use_spec() {
        let story = StoryBuilder::new("mock-start-node")
            .add_speaker("speaker-1", Speaker::new("Speaker 1"))
            .add_node("story-node-1", StoryNode::Dialogue(DialogueBuilder::new("speaker-1").build()))
            .build();
        assert_eq!(story.start().as_str(), "mock-start-node");

        assert_eq!(story.speakers().len(), 1);
        assert_eq!(story.speakers().get("speaker-1").unwrap().name().as_str(), "Speaker 1");

        assert_eq!(story.story_nodes().len(), 1);

        let _ = match story.story_nodes().get("story-node-1").unwrap() {
            StoryNode::Dialogue(dialogue) => {
                assert_eq!(dialogue.speaker().as_str(), "speaker-1");
            }
            StoryNode::Part(_) => panic!("should be a dialogue")
        };
    }
}