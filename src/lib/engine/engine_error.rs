use std::{error::Error, fmt::{Formatter, Display}};

#[derive(Debug, PartialEq)]
pub enum EngineErrorType {
    NoStory,
    NoCurrent,
    NoChoiceArg,
    NoDialogue,
    NoQuotes,
    NoNextClosure,
    NoChangeContextClosure,
    ChoiceDne,
    StoryNodeDne,
    CurrentNoDialogue,
    NoStart,
}

impl Display for EngineErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            EngineErrorType::NoCurrent => "Story does not have a current story node",
            EngineErrorType::NoStory => "Engine does not have a story",
            EngineErrorType::NoChoiceArg => "Choice was not provided to engine",
            EngineErrorType::NoDialogue => "Story does not have dialogues",
            EngineErrorType::NoQuotes => "Dialogue does not have quotes",
            EngineErrorType::NoNextClosure => "StoryLink does not have a next closure",
            EngineErrorType::ChoiceDne => "Choice does not exist in dialogue",
            EngineErrorType::StoryNodeDne => "StoryNode does not exist in story",
            EngineErrorType::CurrentNoDialogue => "Story's current node does not have a dialogue",
            EngineErrorType::NoStart => "Story does not have a starting story node",
            EngineErrorType::NoChangeContextClosure => "StoryLink does not have a change context closure"
        };
        write!(f, "Error - {}", error_message)
    }
}

#[derive(Debug)]
pub struct EngineError {
    error: EngineErrorType,
    description: String,
}

impl EngineError {
    pub fn new(error: EngineErrorType) -> Self {
        Self { description: error.to_string(), error }
    }
    pub fn error(&self) -> &EngineErrorType {
        &self.error
    }
    pub fn description(&self) -> &str {
        &self.description
    }
}

impl Display for EngineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "EngineError {{ description: {} }}", self.description)
    }
}


impl Error for EngineError {
    fn description(&self) -> &str {
        &self.description
    }
}