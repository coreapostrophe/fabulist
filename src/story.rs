use std::fmt::{Display, Formatter};

use crate::story::story_node::context::Context;
use crate::story::story_node::dialogue::speaker::speakers::Speakers;
use crate::story::story_node::story_nodes::StoryNodes;

pub mod story_node;
pub mod story_link;
pub mod story_builder;

pub struct Story {
    story_context: Context,
    speakers: Speakers,
    start: String,
    current: Option<String>,
    story_nodes: StoryNodes,
}

impl Story {
    pub fn current(&self) -> &Option<String> {
        &self.current
    }
    pub fn story_context(&self) -> &Context {
        &self.story_context
    }
    pub fn story_nodes(&self) -> &StoryNodes {
        &self.story_nodes
    }
    pub fn speakers(&self) -> &Speakers {
        &self.speakers
    }
    pub fn start(&self) -> &String {
        &self.start
    }

    pub fn mut_story_context(&mut self) -> &mut Context {
        &mut self.story_context
    }
    pub fn mut_story_nodes(&mut self) -> &mut StoryNodes {
        &mut self.story_nodes
    }
    pub fn mut_speakers(&mut self) -> &mut Speakers {
        &mut self.speakers
    }

    pub fn set_current(&mut self, current: String) {
        self.current = Some(current);
    }
}

impl Display for Story {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Story {{story_context: {}, speakers: {}, start: {}, story_nodes: {}}}",
            self.story_context,
            self.speakers,
            self.start,
            self.story_nodes
        )
    }
}

#[cfg(test)]
mod story_tests {
    use crate::story::story_builder::StoryBuilder;
    use crate::story::story_node::context::ContextValue;
    use crate::story::story_node::dialogue::dialogue_builder::DialogueBuilder;
    use crate::story::story_node::dialogue::quote::quote_builder::QuoteBuilder;

    #[test]
    fn it_displays() {
        let story = StoryBuilder::new(String::from("dialogue-1"))
            .add_context(String::from("count"), ContextValue::Integer(2))
            .add_story_node(
                String::from("dialogue-1"),
                DialogueBuilder::new(String::from("core"))
                    .choice(0)
                    .add_quote(
                        QuoteBuilder::new(String::from("I'm a sample dialogue!"))
                            .response(String::from("Yo! I'm a sample dialogue!"))
                            .build()
                    )
                    .add_quote(
                        QuoteBuilder::new(String::from("I'm another sample dialogue!"))
                            .response(String::from("Yo! I'm another sample dialogue!"))
                            .build()
                    )
                    .build(),
            )
            .build();
        assert_eq!(
            story.to_string(),
            "Story {story_context: Context {{key: count, value: 2}}, \
            speakers: StoryNodes {}, start: dialogue-1, \
            story_nodes: StoryNodes {{key: dialogue-1, \
            value: StoryNode {Dialogue {speaker: \"core\", \
            quotes: [Quote {text: \"I'm a sample dialogue!\", \
            response: \"Yo! I'm a sample dialogue!\"}, \
            Quote {text: \"I'm another sample dialogue!\", \
            response: \"Yo! I'm another sample dialogue!\"}], choice: 0}}}}}"
        );
    }
}
