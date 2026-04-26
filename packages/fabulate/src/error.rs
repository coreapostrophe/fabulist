use std::io;

use fabc::{Error as CompilerError, StoryRuntimeError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Runtime(#[from] StoryRuntimeError),
    #[error(transparent)]
    Compiler(#[from] CompilerError),
}

pub type Result<T> = std::result::Result<T, Error>;
