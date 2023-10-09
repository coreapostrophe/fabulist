use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Add;

use crate::story::story_node::RcStoryNode;

pub struct StoryNodes(pub HashMap<String, RcStoryNode>);

impl StoryNodes {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn insert(&mut self, key: String, value: RcStoryNode) {
        self.0.insert(key, value);
    }
    pub fn get(&self, key: &str) -> Option<&RcStoryNode> {
        self.0.get(key)
    }
    pub fn get_mut(&mut self, key: &str) -> Option<&mut RcStoryNode> {
        self.0.get_mut(key)
    }
}

impl Display for StoryNodes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let story_nodes_len = self.0.len();
        let story_nodes_string = self.0.iter().enumerate().fold(
            String::new(),
            |string_builder, (index, (key, rc_story_node))| {
                let rc_story_node_value_str = rc_story_node.borrow().to_string();
                let ending_comma =
                    if index == story_nodes_len - 1 { String::from("") } else { String::from(", ") };
                let entry_str = format!(
                    "{{key: {}, value: {}}}{}",
                    key,
                    rc_story_node_value_str,
                    ending_comma
                );
                string_builder.add(&entry_str)
            },
        );
        write!(f, "StoryNodes {{{}}}", story_nodes_string)
    }
}

#[cfg(test)]
mod story_nodes_tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::story::story_node::part::part_builder::PartBuilder;

    use super::*;

    #[test]
    fn it_displays() {
        let mut story_nodes = StoryNodes::new();
        story_nodes.insert(
            String::from("part"),
            Rc::new(
                RefCell::new(
                    PartBuilder::new()
                        .add_story_node(String::from("sample-dialogue"))
                        .build()
                )
            ),
        );
        assert_eq!(
            story_nodes.to_string(),
            "StoryNodes {{key: part, value: StoryNode {Part [sample-dialogue]}}}"
        );
    }
}
