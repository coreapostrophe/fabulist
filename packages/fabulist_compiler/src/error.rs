use std::{error::Error, fmt::Display};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Source(u32, u32);

#[allow(dead_code)]
#[derive(Debug)]
pub enum CompilerErrorType {
    PlaceholderError(Source),
}

impl Display for CompilerErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            Self::PlaceholderError(source) => {
                format!("[line {}:{}] example error here", source.0, source.1)
            }
        };
        write!(f, "{}", error_message)
    }
}

impl Error for CompilerErrorType {}
