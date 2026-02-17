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
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{ast::expr::literal::Literal, Parser};

    #[test]
    fn parses_literals() {
        let literal =
            Parser::parse_ast_str::<Literal>("42").expect("Failed to parse literal");
        assert_debug_snapshot!("literal_number", literal);

        let literal = Parser::parse_ast_str::<Literal>("\"hello\"")
            .expect("Failed to parse literal");
        assert_debug_snapshot!("literal_string", literal);

        let literal = Parser::parse_ast_str::<Literal>("true").expect("Failed to parse literal");
        assert_debug_snapshot!("literal_true", literal);

        let literal = Parser::parse_ast_str::<Literal>("false").expect("Failed to parse literal");
        assert_debug_snapshot!("literal_false", literal);

        let literal = Parser::parse_ast_str::<Literal>("none").expect("Failed to parse literal");
        assert_debug_snapshot!("literal_none", literal);
    }
}
