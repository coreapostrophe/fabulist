use crate::{
    error::CompilerError,
    token::{Token, TokenType},
};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u32,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn identify_token(&self) -> Result<Token, CompilerError> {
        let c = self.source.chars().nth(self.current).unwrap();
        let token_type = match c {
            '[' => Ok(TokenType::LeftBracket),
            ']' => Ok(TokenType::RightBracket),
            '{' => Ok(TokenType::LeftBrace),
            '}' => Ok(TokenType::RightBrace),
            ':' => Ok(TokenType::Colon),
            '=' => {
                if self.match_next('=') {
                    Ok(TokenType::EqualEqual)
                } else {
                    Ok(TokenType::Equal)
                }
            }
            _ => Err(CompilerError::UnexpectedCharacter(self.line)),
        }?;
        Ok(Token::new(token_type))
    }

    fn match_next(&self, expected: char) -> bool {
        let next_char = self.source.chars().nth(self.current + 1);
        match next_char {
            Some(c) => {
                if c == expected {
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_tokens(&mut self) -> Result<(), CompilerError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.tokens.push(self.identify_token()?);
            self.current += 1;
        }

        self.tokens.push(
            Token::new(TokenType::EndOfFile)
                .set_line(self.line)
                .set_lexeme("".to_string()),
        );

        Ok(())
    }
}
