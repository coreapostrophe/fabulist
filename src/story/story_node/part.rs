use std::fmt::{Display, Formatter};

use crate::util::vec_to_str;

pub mod part_builder;

pub struct Part {
    story_nodes: Vec<String>,
}

impl Part {
    pub fn story_nodes(&self) -> &Vec<String> {
        &self.story_nodes
    }
    pub fn mut_story_nodes(&mut self) -> &mut Vec<String> {
        &mut self.story_nodes
    }
    pub fn set_story_nodes(&mut self, story_nodes: Vec<String>) {
        self.story_nodes = story_nodes;
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Part {}", vec_to_str(self.story_nodes()))
    }
}

#[cfg(test)]
mod part_tests {
    use crate::story::story_node::part::part_builder::PartBuilder;

    #[test]
    fn it_displays() {
        let part = PartBuilder::new()
            .add_story_node(String::from("story-node-1"))
            .add_story_node(String::from("story-node-2"))
            .add_story_node(String::from("story-node-3"))
            .build_classic();
        assert_eq!(part.to_string(), "Part [story-node-1, story-node-2, story-node-3]")
    }
}
