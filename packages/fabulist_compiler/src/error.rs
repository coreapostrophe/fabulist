use std::{error::Error, fmt::Display};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Source(u32, u32);

impl Source {
    fn to_tuple(&self) -> (u32, u32) {
        (self.0, self.1)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum CompilerError {
    PlaceholderError(Source),
}

impl CompilerError {
    fn format_error(error: &CompilerError, source: &Source, message: &str) -> String {
        let (line, line_offset) = source.to_tuple();
        format!("line {}:{} {:?} - {}", line, line_offset, error, message)
    }
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            Self::PlaceholderError(source) => Self::format_error(self, source, "placeholder error"),
        };
        write!(f, "{}", error_message)
    }
}

impl Error for CompilerError {}
