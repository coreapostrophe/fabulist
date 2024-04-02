pub struct Part {
    story_nodes: Vec<String>,
}

impl Part {
    pub fn new() -> Self {
        Self {
            story_nodes: Vec::new(),
        }
    }
    pub fn story_nodes(&self) -> &Vec<String> {
        &self.story_nodes
    }
}

pub struct PartBuilder {
    story_nodes: Vec<String>,
}

impl PartBuilder {
    pub fn new() -> Self {
        Self {
            story_nodes: Vec::new(),
        }
    }
    pub fn add_node(mut self, node_key: &str) -> Self {
        self.story_nodes.push(node_key.to_string());
        self
    }
    pub fn build(self) -> Part {
        Part {
            story_nodes: self.story_nodes,
        }
    }
}

#[cfg(test)]
mod part_tests {
    use super::*;

    #[test]
    fn matches_use_spec() {
        let part = PartBuilder::new()
            .add_node("storynode-1")
            .add_node("storynode-2")
            .add_node("storynode-3")
            .build();
        assert_eq!(part.story_nodes().len(), 3);
    }
}
