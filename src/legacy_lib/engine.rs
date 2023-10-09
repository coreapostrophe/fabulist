use std::ops::Deref;

use crate::engine::engine_error::{EngineError, EngineErrorType};
use crate::story::Story;
use crate::story::story_node::{RcStoryNode, StoryNode};

pub mod engine_error;

pub type EngineResult = Result<(String, RcStoryNode), EngineError>;

pub struct Engine {
    story: Option<Story>,
}

impl Engine {
    pub fn new() -> Self {
        Self { story: None }
    }
    pub fn story(&self) -> &Option<Story> {
        &self.story
    }
    pub fn set_story(&mut self, story: Story) {
        self.story = Some(story);
    }

    pub fn reset(&mut self) -> Result<(), EngineError> {
        let story = self.story.as_mut()
            .ok_or(EngineError::new(EngineErrorType::NoStory))?;
        story.set_current(story.start().clone());
        Ok(())
    }

    pub fn start(&self) -> EngineResult {
        let story = self.story.as_ref()
            .ok_or(EngineError::new(EngineErrorType::NoStory))?;
        let start_node_key = story.start();
        let start_node = story.story_nodes().get(start_node_key)
            .ok_or(EngineError::new(EngineErrorType::StoryNodeDne))?;
        Ok((start_node_key.clone(), start_node.clone()))
    }

    pub fn next(&mut self, choice: Option<usize>) -> EngineResult {
        let story = self.story.as_mut()
            .ok_or(EngineError::new(EngineErrorType::NoStory))?;
        let current_node_key = match story.current() {
            Some(current_node) => current_node,
            _ => story.start()
        };
        let story_context = story.story_context();
        let get_node_from_key = |node_key: String|
                                 -> Result<RcStoryNode, EngineError> {
            Ok(story.story_nodes().get(&node_key)
                .ok_or(EngineError::new(EngineErrorType::StoryNodeDne))?.clone())
        };

        // Unwraps node to find the first dialogue it contains
        let node_dialogue = {
            let mut search_stack = vec![current_node_key.clone()];
            let mut result: Option<RcStoryNode> = None;
            while result.is_none() {
                let first_query = search_stack.first()
                    .ok_or(EngineError::new(EngineErrorType::CurrentNoDialogue))?;
                let node = get_node_from_key(first_query.clone())?;
                let ref_node = node.borrow();
                match ref_node.deref() {
                    StoryNode::Dialogue(_) => result = Some(node.clone()),
                    StoryNode::Part(part) => {
                        for story_node in part.story_nodes().iter().rev() {
                            search_stack.insert(1, story_node.clone());
                        }
                    }
                }
            }
            result.ok_or(EngineError::new(EngineErrorType::CurrentNoDialogue))?
        };

        let ref_node_dialogue = node_dialogue.borrow();

        match ref_node_dialogue.deref() {
            // If the unwrapped node is a dialogue, proceed with the next function.
            StoryNode::Dialogue(dialogue) => {
                // Gets chosen quote from dialogue
                let quote = if dialogue.has_choices() {
                    let choice_index = choice
                        .ok_or(EngineError::new(EngineErrorType::NoChoiceArg))?;
                    dialogue.quotes().0.get(choice_index)
                        .ok_or(EngineError::new(EngineErrorType::ChoiceDne))?
                } else {
                    dialogue.first_quote()
                        .ok_or(EngineError::new(EngineErrorType::NoQuotes))?
                };

                // Parses the quote's next closure
                let next_closure = quote.story_link().next()
                    .ok_or(EngineError::new(EngineErrorType::NoNextClosure))?;

                // Gets the next node key from the next closure and creates result
                let next_node_key = next_closure(story_context);
                let next_node = story.story_nodes().get(&next_node_key)
                    .ok_or(EngineError::new(EngineErrorType::StoryNodeDne))?.clone();
                let result = (next_node_key.clone(), next_node.clone());

                // Changes the context with the quote's change context closure
                match quote.story_link().change_context() {
                    Some(change_context_closure) => {
                        change_context_closure(story.mut_story_context());
                    }
                    _ => ()
                }

                story.set_current(next_node_key);
                Ok(result)
            }
            // If the unwrapped node is anything else, throw an error.
            _ => return Err(EngineError::new(EngineErrorType::NoDialogue))
        }
    }
}

#[cfg(test)]
mod engine_tests {
    use crate::story::story_builder::StoryBuilder;
    use crate::story::story_node::context::ContextValue;
    use crate::story::story_node::dialogue::dialogue_builder::DialogueBuilder;
    use crate::story::story_node::dialogue::quote::quote_builder::QuoteBuilder;

    use super::*;

    #[test]
    fn it_constructs() {
        let engine = Engine::new();
        assert!(engine.story().is_none());
    }

    #[test]
    fn start_works() {
        let story = StoryBuilder::new(String::from("dialogue-1"))
            .add_story_node(
                String::from("dialogue-1"),
                DialogueBuilder::new(String::from("core"))
                    .build(),
            )
            .build();

        let mut engine = Engine::new();
        engine.set_story(story);

        let (node_key, _) = engine.start().unwrap();
        assert_eq!(node_key, "dialogue-1");
    }

    #[test]
    fn next_works() {
        let story = StoryBuilder::new(String::from("dialogue-1"))
            .add_story_node(
                String::from("dialogue-1"),
                DialogueBuilder::new(String::from("core"))
                    .add_quote(
                        QuoteBuilder::new(String::from("Hello!"))
                            .next(|_| String::from("dialogue-2"))
                            .build()
                    )
                    .add_quote(
                        QuoteBuilder::new(String::from("Homie!"))
                            .next(|_| String::from("dialogue-3"))
                            .build()
                    )
                    .build(),
            )
            .add_story_node(
                String::from("dialogue-2"),
                DialogueBuilder::new(String::from("jose"))
                    .add_quote(
                        QuoteBuilder::new(String::from("Hi there!"))
                            .build()
                    )
                    .build(),
            )
            .add_story_node(
                String::from("dialogue-3"),
                DialogueBuilder::new(String::from("jose"))
                    .add_quote(
                        QuoteBuilder::new(String::from("Who are you?"))
                            .next(|_| String::from("dialogue-4"))
                            .build()
                    )
                    .build(),
            )
            .build();

        let mut engine = Engine::new();
        engine.set_story(story);

        let (node_1_key, _) = engine.next(Some(0)).unwrap();
        let _ = engine.reset();
        let (node_2_key, _) = engine.next(Some(1)).unwrap();

        assert_eq!(node_1_key, "dialogue-2");
        assert_eq!(node_2_key, "dialogue-3");
    }

    #[test]
    fn next_with_context_works() {
        let story = StoryBuilder::new(String::from("dialogue-0"))
            .add_context(String::from("is-loved"), ContextValue::Bool(false))
            .add_story_node(
                String::from("dialogue-0"),
                DialogueBuilder::new(String::from("girl"))
                    .add_quote(
                        QuoteBuilder::new(String::from("Don't you know?"))
                            .next(|_| String::from("dialogue-1"))
                            .build()
                    )
                    .add_quote(
                        QuoteBuilder::new(String::from("I'm sure of it."))
                            .next(|_| String::from("dialogue-1"))
                            .change_context(|c| {
                                *c.get_mut("is-loved")
                                    .expect("Story to have an `is-loved` context") =
                                    ContextValue::Bool(true);
                            })
                            .build()
                    )
                    .build(),
            )
            .add_story_node(
                String::from("dialogue-1"),
                DialogueBuilder::new(String::from("girl"))
                    .add_quote(
                        QuoteBuilder::new(String::from("I love you."))
                            .next(|c| {
                                let is_loved = c.get("is-loved")
                                    .expect("Story to have an `is-loved` context");
                                return match is_loved {
                                    ContextValue::Bool(is_loved_bool) => {
                                        if is_loved_bool.clone() {
                                            String::from("dialogue-2")
                                        } else {
                                            String::from("dialogue-3")
                                        }
                                    }
                                    _ => String::from("dialogue-3")
                                };
                            })
                            .build()
                    )
                    .build(),
            )
            .add_story_node(
                String::from("dialogue-2"),
                DialogueBuilder::new(String::from("boy"))
                    .add_quote(
                        QuoteBuilder::new(String::from("I love you too!"))
                            .build()
                    )
                    .build(),
            )
            .add_story_node(
                String::from("dialogue-3"),
                DialogueBuilder::new(String::from("boy"))
                    .add_quote(
                        QuoteBuilder::new(String::from("I'm... sorry."))
                            .build()
                    )
                    .build(),
            )
            .build();


        let mut engine = Engine::new();
        engine.set_story(story);

        let _ = engine.next(Some(0)).unwrap();
        let (node_key, _) = engine.next(Some(0)).unwrap();
        assert_eq!(node_key, "dialogue-3");

        let _ = engine.reset();
        let _ = engine.next(Some(1)).unwrap();
        let (node_key, _) = engine.next(None).unwrap();
        assert_eq!(node_key, "dialogue-2");
    }
}
