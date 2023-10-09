use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

use crate::story::story_node::dialogue::Dialogue;
use crate::story::story_node::part::Part;

pub mod context;
pub mod dialogue;
pub mod part;
pub mod story_nodes;

pub enum StoryNode {
    Dialogue(Dialogue),
    Part(Part),
}

impl Display for StoryNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StoryNode::Dialogue(dialogue) => write!(f, "StoryNode {{{}}}", dialogue),
            StoryNode::Part(part) => write!(f, "StoryNode {{{}}}", part),
        }
    }
}

pub type RcStoryNode = Rc<RefCell<StoryNode>>;

#[cfg(test)]
mod story_node_tests {
    use crate::story::story_node::dialogue::dialogue_builder::DialogueBuilder;
    use crate::story::story_node::part::part_builder::PartBuilder;

    #[test]
    fn it_displays() {
        let story_node_dialogue = DialogueBuilder::new(String::from("core")).build();
        let story_node_part = PartBuilder::new()
            .add_story_node(String::from("sample-dialogue"))
            .build();
        assert_eq!(
            story_node_dialogue.to_string(),
            "StoryNode {Dialogue {speaker: \"core\", quotes: []}}"
        );
        assert_eq!(
            story_node_part.to_string(),
            "StoryNode {Part [sample-dialogue]}"
        );
    }
}
