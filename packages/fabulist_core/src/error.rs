use crate::story::reference::ListKey;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Required choice index was not provided.")]
    ChoiceWasNotProvided,
    #[error("Provided choice index `{index}` is invalid.")]
    InvalidChoice { index: usize },
    #[error("Dialogue index `{dialogue_index}` does not exist in part.")]
    DialogueDoesNotExist {
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

pub type Result<T> = std::result::Result<T, Error>;
