use std::collections::HashMap;

use crate::story::{context::Context, Story, story_link::{ChangeContextClosure, NextClosure}, story_node::StoryNode};

use self::engine_error::{EngineError, EngineErrorType};

pub mod engine_error;

pub type EngineResult = Result<String, EngineError>;

pub struct Engine {
    story: Option<Story>,
    context: Context,
    current_node: Option<String>,
    choices: HashMap<String, usize>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            story: None,
            choices: HashMap::new(),
            context: Context::new(),
            current_node: None,
        }
    }

    pub fn story(&self) -> Option<&Story> {
        match self.story {
            Some(ref story) => Some(story),
            None => None
        }
    }
    pub fn mut_story(&mut self) -> Option<&mut Story> {
        match &mut self.story {
            Some(ref mut story) => Some(story),
            None => None
        }
    }
    pub fn set_story(&mut self, story: Story) {
        self.story = Some(story);
    }
    pub fn choices(&self) -> &HashMap<String, usize> {
        &self.choices
    }
    pub fn mut_choices(&mut self) -> &mut HashMap<String, usize> {
        &mut self.choices
    }
    pub fn context(&self) -> &Context {
        &self.context
    }
    pub fn mut_context(&mut self) -> &mut Context {
        &mut self.context
    }
    pub fn current_node(&self) -> Option<&String> {
        match self.current_node {
            Some(ref current_node) => Some(current_node),
            None => None
        }
    }
    pub fn set_current_node(&mut self, node_key: &str) {
        self.current_node = Some(node_key.to_string());
    }

    pub fn start(&mut self) -> EngineResult {
        let new_current_node = {
            match self.current_node() {
                Some(current_node) => current_node.clone(),
                None => {
                    let story = self.mut_story().ok_or(EngineError::new(EngineErrorType::NoStory))?;
                    story.start().clone()
                }
            }
        };
        self.set_current_node(&new_current_node);
        Ok(new_current_node)
    }

    pub fn reset(&mut self) -> EngineResult {
        let story = self.story().ok_or(EngineError::new(EngineErrorType::NoStory))?;
        let start_node = story.start().clone();
        self.set_current_node(&start_node);
        Ok(start_node)
    }

    pub fn get_node_dialogue(&self, node_key: &str) -> Result<String, EngineError> {
        let story = self.story().ok_or(EngineError::new(EngineErrorType::NoStory))?;
        let mut search_stack = vec![node_key.to_string()];
        let mut result: Option<String> = None;
        while result.is_none() {
            let first_query = search_stack.first()
                .ok_or(EngineError::new(EngineErrorType::NoCurrentDialogue))?;
            let node = story.story_nodes().get(first_query);
            match node.ok_or(EngineError::new(EngineErrorType::StoryNodeDne))? {
                StoryNode::Dialogue(_) => result = Some(first_query.clone()),
                StoryNode::Part(part) => {
                    for story_node in part.story_nodes().iter().rev() {
                        search_stack.insert(1, story_node.clone());
                    }
                }
            }
        }
        Ok(result.ok_or(EngineError::new(EngineErrorType::NoDialogue))?)
    }

    pub fn get_current_dialogue_key(&self) -> Result<String, EngineError> {
        let current_node_key = self.current_node().ok_or(EngineError::new(EngineErrorType::NoCurrent))?;
        self.get_node_dialogue(current_node_key)
    }

    pub fn next(&mut self, choice: Option<usize>) -> EngineResult {
        let node_dialogue_key = self.get_current_dialogue_key()?;

        let (
            next_closure,
            change_context_closure,
            has_choices
        ): (Option<NextClosure>, Option<ChangeContextClosure>, bool) = {
            let story = self.story().ok_or(EngineError::new(EngineErrorType::NoStory))?;
            let node_dialogue = story.story_nodes().get(&node_dialogue_key)
                .ok_or(EngineError::new(EngineErrorType::StoryNodeDne))?;
            let result = match node_dialogue {
                StoryNode::Dialogue(dialogue) => {
                    let has_choices = dialogue.quotes().len() > 1;
                    let quote = if has_choices {
                        match choice {
                            Some(choice_index) => Ok(dialogue.quotes().get(choice_index)
                                .ok_or(EngineError::new(EngineErrorType::QuoteDne))?),
                            None => Err(EngineError::new(EngineErrorType::MissingChoiceArg))
                        }
                    } else {
                        Ok(dialogue.first_quote()
                            .ok_or(EngineError::new(EngineErrorType::NoQuotes))?)
                    }?;
                    let next_closure = match quote.story_link().next() {
                        Some(next_closure) => Some(next_closure.clone()),
                        None => None
                    };
                    let change_context_closure = match quote.story_link().change_context() {
                        Some(change_context_closure) => Some(change_context_closure.clone()),
                        None => None
                    };
                    Ok((next_closure, change_context_closure, has_choices))
                }
                _ => Err(EngineError::new(EngineErrorType::NoDialogue))
            }?;
            result
        };

        let next_node_key = match next_closure {
            Some(next_closure) => Ok(next_closure(self.context())),
            None => Err(EngineError::new(EngineErrorType::NoNextClosure))
        }?;
        match change_context_closure {
            Some(change_context_closure) => change_context_closure(self.mut_context()),
            None => ()
        }
        if has_choices {
            let choice_index = choice.ok_or(EngineError::new(EngineErrorType::MissingChoiceArg))?;
            self.mut_choices().insert(node_dialogue_key, choice_index);
        };
        self.set_current_node(&next_node_key);
        Ok(next_node_key)
    }
}

#[cfg(test)]
mod engine_tests {
    use crate::story::{context::ContextValue, story_node::{dialogue::{DialogueBuilder, quote::QuoteBuilder}, StoryNode}, StoryBuilder};

    use super::*;

    #[test]
    fn story_starts() {
        let story = StoryBuilder::new("dialogue-1")
            .add_node("dialogue-1", StoryNode::Dialogue(DialogueBuilder::new("core").build()))
            .build();

        let mut engine = Engine::new();
        engine.set_story(story);

        let node_key = engine.start().unwrap();
        assert_eq!(node_key, "dialogue-1");
    }

    #[test]
    fn next_works() {
        let story = StoryBuilder::new("dialogue-1")
            .add_node(
                "dialogue-1",
                StoryNode::Dialogue(
                    DialogueBuilder::new("core")
                        .add_quote(
                            QuoteBuilder::new("Hello!")
                                .set_next(|_| "dialogue-2".to_string())
                                .build()
                        )
                        .add_quote(
                            QuoteBuilder::new("Homie!")
                                .set_next(|_| "dialogue-3".to_string())
                                .build()
                        )
                        .build()
                ),
            )
            .add_node(
                "dialogue-2",
                StoryNode::Dialogue(
                    DialogueBuilder::new("jose")
                        .add_quote(QuoteBuilder::new("Hi there!").build())
                        .build()
                ),
            )
            .add_node(
                "dialogue-3",
                StoryNode::Dialogue(
                    DialogueBuilder::new("jose")
                        .add_quote(
                            QuoteBuilder::new("Who are you?")
                                .set_next(|_| String::from("dialogue-4"))
                                .build()
                        )
                        .build(),
                ),
            )
            .build();

        let mut engine = Engine::new();
        engine.set_story(story);

        let _ = engine.start();
        let node_1_key = engine.next(Some(0)).unwrap();
        let _ = engine.reset();
        let node_2_key = engine.next(Some(1)).unwrap();

        assert_eq!(node_1_key, "dialogue-2");
        assert_eq!(node_2_key, "dialogue-3");
    }

    #[test]
    fn next_with_context_works() {
        let mut engine = Engine::new();
        engine.mut_context().insert("is-loved".to_string(), ContextValue::Bool(false));

        let story = StoryBuilder::new("dialogue-0")
            .add_node(
                "dialogue-0",
                StoryNode::Dialogue(
                    DialogueBuilder::new("girl")
                        .add_quote(
                            QuoteBuilder::new("Don't you know?")
                                .set_next(|_| "dialogue-1".to_string())
                                .build()
                        )
                        .add_quote(
                            QuoteBuilder::new("I'm sure of it.")
                                .set_next(|_| "dialogue-1".to_string())
                                .set_change_context(|context| {
                                    *context.get_mut("is-loved")
                                        .expect("Story to have an `is-loved` context") =
                                        ContextValue::Bool(true);
                                })
                                .build()
                        )
                        .build(),
                ),
            )
            .add_node(
                "dialogue-1",
                StoryNode::Dialogue(
                    DialogueBuilder::new("girl")
                        .add_quote(
                            QuoteBuilder::new("I love you.")
                                .set_next(|context| {
                                    let is_loved = context.get("is-loved")
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
                ),
            )
            .add_node(
                "dialogue-2",
                StoryNode::Dialogue(
                    DialogueBuilder::new("boy")
                        .add_quote(QuoteBuilder::new("I love you too!").build())
                        .build(),
                ),
            )
            .add_node(
                "dialogue-3",
                StoryNode::Dialogue(
                    DialogueBuilder::new("boy")
                        .add_quote(QuoteBuilder::new("I'm... sorry.").build())
                        .build(),
                ),
            )
            .build();

        engine.set_story(story);

        let _ = engine.start();
        let _ = engine.next(Some(0));
        let node_key = engine.next(None).unwrap();
        assert_eq!(node_key, "dialogue-3");

        let _ = engine.reset();
        let _ = engine.next(Some(1));
        let node_key = engine.next(None).unwrap();
        assert_eq!(node_key, "dialogue-2");
    }
}