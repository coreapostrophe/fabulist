use std::{error::Error, fmt::{Formatter, Display}};

#[derive(Debug, PartialEq)]
pub enum EngineErrorType {
    NoStory,
    NoStart,
    NoQuotes,
    NoCurrent,
    NoDialogue,
    NoNextClosure,
    NoCurrentDialogue,
    NoChangeContextClosure,

    MissingChoiceArg,

    QuoteDne,
    ChoiceDne,
    StoryNodeDne,
}

impl Display for EngineErrorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            EngineErrorType::NoStory => "Engine does not have a story",
            EngineErrorType::NoStart => "Story does not have a start node",
            EngineErrorType::NoQuotes => "Dialogue does not have any quotes",
            EngineErrorType::NoCurrent => "Story does not have a current node",
            EngineErrorType::NoDialogue => "Story does not have a dialogue",
            EngineErrorType::NoNextClosure => "StoryLink does not have a next closure",
            EngineErrorType::NoCurrentDialogue => "Story's current node does not have a dialogue",
            EngineErrorType::NoChangeContextClosure => "StoryLink does not have a change context closure",

            EngineErrorType::MissingChoiceArg => "Consumer did not provide a choice argument to next",

            EngineErrorType::QuoteDne => "Quote does not exist in dialogue",
            EngineErrorType::ChoiceDne => "Choice does not exist in dialogue",
            EngineErrorType::StoryNodeDne => "StoryNode does not exist in story"
        };
        write!(f, "{}", error_message)
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