use fabc_error::{kind::CompileErrorKind, Error};
use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{ast::NodeInfo, expect_token, Parsable, Parser};

#[derive(Debug, PartialEq)]
pub enum Literal {
    Boolean { info: NodeInfo, value: bool },
    String { info: NodeInfo, value: String },
    Number { info: NodeInfo, value: f64 },
    None { info: NodeInfo },
}

impl Literal {
    pub fn info(&self) -> &NodeInfo {
        match self {
            Literal::Boolean { info, .. }
            | Literal::String { info, .. }
            | Literal::Number { info, .. }
            | Literal::None { info } => info,
        }
    }
}

impl Parsable for Literal {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        match parser.peek() {
            TokenKind::Keyword(KeywordKind::True) => {
                parser.advance();

                Ok(Literal::Boolean {
                    info: NodeInfo {
                        id: parser.assign_id(),
                        span: parser.previous_token().into(),
                    },
                    value: true,
                })
            }
            TokenKind::Keyword(KeywordKind::False) => {
                parser.advance();

                Ok(Literal::Boolean {
                    info: NodeInfo {
                        id: parser.assign_id(),
                        span: parser.previous_token().into(),
                    },
                    value: false,
                })
            }
            TokenKind::String(_) => {
                let value = expect_token!(parser, TokenKind::String, "string literal")?;

                Ok(Literal::String {
                    info: NodeInfo {
                        id: parser.assign_id(),
                        span: parser.previous_token().into(),
                    },
                    value,
                })
            }
            TokenKind::Number(_) => {
                let value = expect_token!(parser, TokenKind::Number, "number literal")?;

                Ok(Literal::Number {
                    info: NodeInfo {
                        id: parser.assign_id(),
                        span: parser.previous_token().into(),
                    },
                    value,
                })
            }
            TokenKind::Keyword(KeywordKind::None) => {
                parser.advance();

                Ok(Literal::None {
                    info: NodeInfo {
                        id: parser.assign_id(),
                        span: parser.previous_token().into(),
                    },
                })
            }
            _ => Err(Error::new(
                CompileErrorKind::UnrecognizedLiteral {
                    literal: parser.previous().to_string(),
                },
                parser.peek_token(),
            )),
        }
    }
}

#[cfg(test)]
mod literal_tests {
    use fabc_lexer::Lexer;

    use crate::{ast::expr::literal::Literal, Parser};

    #[test]
    fn parses_literals() {
        let source = "42";
        let tokens = Lexer::tokenize(source);
        let literal = Parser::parse_ast::<Literal>(&tokens).expect("Failed to parse literal");
        match literal {
            Literal::Number { value, .. } => assert_eq!(value, 42.0),
            _ => panic!("Expected Number literal"),
        }

        let source = "\"hello\"";
        let tokens = Lexer::tokenize(source);
        let literal = Parser::parse_ast::<Literal>(&tokens).expect("Failed to parse literal");
        match literal {
            Literal::String { value, .. } => assert_eq!(value, "hello".to_string()),
            _ => panic!("Expected String literal"),
        }

        let source = "true";
        let tokens = Lexer::tokenize(source);
        let literal = Parser::parse_ast::<Literal>(&tokens).expect("Failed to parse literal");
        match literal {
            Literal::Boolean { value, .. } => assert!(value),
            _ => panic!("Expected Boolean literal"),
        }

        let source = "false";
        let tokens = Lexer::tokenize(source);
        let literal = Parser::parse_ast::<Literal>(&tokens).expect("Failed to parse literal");
        match literal {
            Literal::Boolean { value, .. } => assert!(!value),
            _ => panic!("Expected Boolean literal"),
        }

        let source = "none";
        let tokens = Lexer::tokenize(source);
        let literal = Parser::parse_ast::<Literal>(&tokens).expect("Failed to parse literal");
        match literal {
            Literal::None { .. } => {}
            _ => panic!("Expected None literal"),
        }
    }
}
