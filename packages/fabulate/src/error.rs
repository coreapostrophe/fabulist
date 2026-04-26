use std::io;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Runtime(#[from] fabc::StoryRuntimeError),
    #[error(transparent)]
    Compiler(#[from] fabc::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
