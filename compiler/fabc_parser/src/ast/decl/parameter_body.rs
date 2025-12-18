use fabc_lexer::tokens::Token;

use crate::{error::Error, Parsable};

#[derive(Debug, PartialEq)]
pub struct ParameterBodyDecl {
    pub parameters: Vec<String>,
}

impl Parsable for ParameterBodyDecl {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
        parser.consume(Token::LeftParen)?;

        let mut parameters = Vec::new();
        if parser.peek() != &Token::RightParen {
            loop {
                if let Token::Identifier(param) = parser.advance() {
                    parameters.push(param.to_string());
                } else {
                    return Err(Error::ExpectedFound {
                        expected: "identifier".to_string(),
                        found: parser.peek().to_string(),
                    });
                }

                if let Token::Comma = parser.peek() {
                    parser.consume(Token::Comma)?;
                } else {
                    break;
                }
            }
        }

        parser.consume(Token::RightParen)?;

        Ok(ParameterBodyDecl { parameters })
    }
}
