use crate::{
    error::{CompilerError, Source},
    token::{Token, TokenType},
};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start_index: usize,
    current_char_index: usize,
    current_line_number: u32,
    current_line_offset: u32,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: vec![],
            start_index: 0,
            current_char_index: 0,
            current_line_number: 1,
            current_line_offset: 1,
        }
    }

    fn identify_token(&self) -> Result<(Token, bool), CompilerError> {
        let c = self.source.chars().nth(self.current_char_index).unwrap();
        let mut is_new_line = false;

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
            '\n' => {
                is_new_line = true;
                Ok(TokenType::NewLine)
            }
            _ => Err(CompilerError::UnexpectedCharacter(Source::new(
                self.current_line_number,
                self.current_line_offset,
            ))),
        }?;
        Ok((Token::new(token_type), is_new_line))
    }

    fn match_next(&self, expected: char) -> bool {
        let next_char = self.source.chars().nth(self.current_char_index + 1);
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
        self.current_char_index >= self.source.len()
    }

    fn scan_tokens(&mut self) -> Result<(), CompilerError> {
        while !self.is_at_end() {
            self.start_index = self.current_char_index;

            let (token, is_new_line) = self.identify_token()?;
            self.tokens.push(token);

            self.current_char_index += 1;

            if is_new_line {
                self.current_line_number += 1;
                self.current_line_offset = 0;
            } else {
                self.current_line_offset += 1;
            }
        }

        self.tokens.push(
            Token::new(TokenType::EndOfFile)
                .set_line(self.current_line_number)
                .set_lexeme("".to_string()),
        );

        Ok(())
    }
}
