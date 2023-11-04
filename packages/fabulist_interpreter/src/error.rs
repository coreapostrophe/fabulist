use std::{error::Error, fmt::Display};

#[allow(dead_code)]
#[derive(Debug)]
pub enum CompilerError {
    UnexpectedCharacter(u32),
}

impl CompilerError {
    fn format_error(error: &CompilerError, line: &u32, message: &str) -> String {
        format!("line {} {:?} - {}", line, error, message)
    }
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            Self::UnexpectedCharacter(line) => {
                Self::format_error(self, line, "unexpected character")
            }
        };
        write!(f, "{}", error_message)
    }
}

impl Error for CompilerError {}
