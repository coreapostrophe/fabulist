use super::StoryNode;

pub struct Part {
    story_nodes: Vec<*mut StoryNode>,
}

impl Part {
    pub fn new() -> Self {
        Self {
            story_nodes: Vec::new(),
        }
    }
    pub fn story_nodes(&self) -> &Vec<*mut StoryNode> {
        &self.story_nodes
    }
}

pub struct PartBuilder {
    story_nodes: Vec<*mut StoryNode>,
}

impl PartBuilder {
    pub fn new() -> Self {
        Self {
            story_nodes: Vec::new(),
        }
    }
    pub fn add_node(mut self, node: *mut StoryNode) -> Self {
        self.story_nodes.push(node);
        self
    }
    pub fn build(self) -> Part {
        Part {
            story_nodes: self.story_nodes,
        }
    }
}
