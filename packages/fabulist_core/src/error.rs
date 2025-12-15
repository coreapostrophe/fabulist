#[cfg(feature = "parsing")]
use fabulist_lang::error::{PestParsingError, RuntimeError};

use crate::story::reference::ListKey;

#[derive(thiserror::Error, Debug)]
pub enum EngineError {
    #[error("Required choice index was not provided.")]
    ChoiceWasNotProvided,
    #[error("Provided choice index `{index}` is invalid.")]
    InvalidChoice { index: usize },
    #[error("Element `{dialogue_index}` does not exist in part `{part_key}`.")]
    ElementDoesNotExist {
        dialogue_index: usize,
        part_key: String,
    },
    #[error("Part `{key}` does not exist.")]
    PartDoesNotExist { key: ListKey<String> },
    #[error("Part `{key}` does not exist.")]
    CharacterDoesNotExist { key: String },
    #[error("Story was not started.")]
    NotStarted,
    #[error("Story does not have a starting part.")]
    StartDoesNotExist,
    #[error("Story has ended.")]
    EndOfStory,
}

pub type EngineResult<T> = std::result::Result<T, EngineError>;

#[cfg(feature = "parsing")]
#[derive(thiserror::Error, Debug)]
pub enum ParsingError {
    #[error("Story start metadata is required but missing.")]
    StartMetadataRequired,
    #[error("Parsing error: {0}")]
    ParsingError(#[from] Box<PestParsingError>),
    #[error("Metadata evaluation error: {0}")]
    MetaEvaluationError(#[from] RuntimeError),
    #[error("Quote properties is not a valid object.")]
    InvalidQuoteProperties,
    #[error("Next hook shouldn't have any parameters.")]
    QueryNextHasParameters,
    #[error("ChangeContext hook shouldn't have any parameters.")]
    QueryChangeContextHasParameters,
}

#[cfg(feature = "parsing")]
pub type ParsingResult<T> = std::result::Result<T, ParsingError>;
