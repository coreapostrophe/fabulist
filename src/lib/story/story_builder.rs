use std::cell::RefCell;
use std::rc::Rc;

use crate::story::{Speakers, Story};
use crate::story::story_node::context::{Context, ContextValue};
use crate::story::story_node::dialogue::speaker::Speaker;
use crate::story::story_node::story_nodes::StoryNodes;
use crate::story::story_node::StoryNode;

pub struct StoryBuilder {
    story_context: Context,
    speakers: Speakers,
    start: String,
    current: Option<String>,
    story_nodes: StoryNodes,
}

impl StoryBuilder {
    pub fn new(start: String) -> Self {
        Self {
            start,
            story_context: Context::new(),
            speakers: Speakers::new(),
            current: None,
            story_nodes: StoryNodes::new(),
        }
    }
    pub fn add_context(mut self, key: String, context_value: ContextValue) -> StoryBuilder {
        self.story_context.insert(key, context_value);
        self
    }
    pub fn add_speakers(mut self, key: String, speaker: Speaker) -> StoryBuilder {
        self.speakers.insert(key, speaker);
        self
    }
    pub fn add_story_node(mut self, key: String, story_node: StoryNode) -> StoryBuilder {
        self.story_nodes.insert(key, Rc::new(RefCell::new(story_node)));
        self
    }
    pub fn build(self) -> Story {
        Story {
            story_context: self.story_context,
            speakers: self.speakers,
            start: self.start,
            current: self.current,
            story_nodes: self.story_nodes,
        }
    }
}

#[cfg(test)]
mod story_builder_tests {
    use super::*;

    #[test]
    fn it_constructs() {
        let story = StoryBuilder::new(String::from("dialogue-1")).build();
        assert!(story.current().is_none());
        assert_eq!(story.start(), "dialogue-1");
        assert_eq!(story.story_nodes().len(), 0);
        assert_eq!(story.speakers().len(), 0);
        assert_eq!(story.story_context().len(), 0);
    }
}
