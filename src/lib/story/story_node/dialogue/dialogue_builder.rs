use crate::story::story_node::dialogue::{Dialogue, Quotes};
use crate::story::story_node::dialogue::quote::Quote;
use crate::story::story_node::StoryNode;

#[derive(Default)]
pub struct DialogueBuilder {
    speaker: String,
    choice: Option<usize>,
    quotes: Quotes,
}

impl DialogueBuilder {
    pub fn new(speaker: String) -> Self {
        Self { speaker, quotes: Quotes(Vec::new()), choice: None }
    }
    pub fn add_quote(mut self, quote: Quote) -> DialogueBuilder {
        self.quotes.0.push(quote);
        self
    }
    pub fn choice(mut self, quote_index: usize) -> DialogueBuilder {
        self.choice = Some(quote_index);
        self
    }
    pub fn build(self) -> StoryNode {
        StoryNode::Dialogue(
            Dialogue {
                speaker: self.speaker,
                choice: self.choice,
                quotes: self.quotes,
            }
        )
    }
    pub fn build_classic(self) -> Dialogue {
        Dialogue {
            speaker: self.speaker,
            choice: self.choice,
            quotes: self.quotes,
        }
    }
}


#[cfg(test)]
mod dialogue_builder_tests {
    use super::*;

    #[test]
    fn it_builds() {
        let dialogue = DialogueBuilder::new(
            String::from("core")
        ).build_classic();
        assert_eq!(dialogue.speaker(), "core");
        assert_eq!(dialogue.quotes().0.len(), 0);
    }
}
