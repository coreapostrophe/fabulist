#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to read file: {0}")]
    ReadFile(#[from] std::io::Error),
}
