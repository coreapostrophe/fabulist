use std::io::Error as IoError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to read file: {0}")]
    ReadFile(#[from] IoError),
}
