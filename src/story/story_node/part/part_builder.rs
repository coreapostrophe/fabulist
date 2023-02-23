use crate::story::story_node::part::Part;
use crate::story::story_node::StoryNode;

pub struct PartBuilder {
    story_nodes: Vec<String>,
}

impl PartBuilder {
    pub fn new() -> Self {
        Self { story_nodes: Vec::new() }
    }
    pub fn add_story_node (mut self, story_node_key: String) -> PartBuilder {
        self.story_nodes.push(story_node_key);
        self
    }
    pub fn build(self) -> StoryNode {
        StoryNode::Part(
            Part {
                story_nodes: self.story_nodes
            }
        )
    }
    pub fn build_classic(self) -> Part {
        Part {
            story_nodes: self.story_nodes
        }
    }
}


#[cfg(test)]
mod part_builder_tests {
    use super::*;

    #[test]
    pub fn it_builds() {
        let mut part = PartBuilder::new()
            .add_story_node(String::from("story-node-key"))
            .build_classic();

        assert_eq!(part.mut_story_nodes().len(), 1);
        assert_eq!(
            part.mut_story_nodes().first()
                .expect("Part to have at least one (1) story node"),
            "story-node-key"
        );
    }
}
