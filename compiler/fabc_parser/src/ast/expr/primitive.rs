use fabc_error::{kind::CompileErrorKind, Error, Span};
use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{
    ast::{decl::object::ObjectDecl, expr::Expr, stmt::block::BlockStmt, NodeInfo},
    expect_token, Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub enum Primitive {
    Identifier {
        info: NodeInfo,
        name: String,
    },
    Grouping {
        info: NodeInfo,
        expr: Box<Expr>,
    },
    Object {
        info: NodeInfo,
        value: ObjectDecl,
    },
    Closure {
        info: NodeInfo,
        params: Vec<Primitive>,
        body: BlockStmt,
    },
    StoryIdentifier {
        info: NodeInfo,
        name: String,
    },
    Context {
        info: NodeInfo,
    },
}

impl Primitive {
    pub fn info(&self) -> &NodeInfo {
        match self {
            Primitive::Identifier { info, .. } => info,
            Primitive::Grouping { info, .. } => info,
            Primitive::Object { info, .. } => info,
            Primitive::Closure { info, .. } => info,
            Primitive::StoryIdentifier { info, .. } => info,
            Primitive::Context { info } => info,
        }
    }
}

impl Parsable for Primitive {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        match parser.peek() {
            TokenKind::Commat => {
                let start_span = parser.start_span();
                parser.consume(TokenKind::Commat)?;
                let name = expect_token!(parser, TokenKind::Identifier, "identifier")?;
                let end_span = parser.end_span();

                Ok(Primitive::StoryIdentifier {
                    info: NodeInfo {
                        id: parser.assign_id(),
                        span: Span::from((start_span, end_span)),
                    },
                    name,
                })
            }
            TokenKind::Identifier(_) => {
                let name = expect_token!(parser, TokenKind::Identifier, "identifier")?;

                Ok(Primitive::Identifier {
                    info: NodeInfo {
                        id: parser.assign_id(),
                        span: Span::from(parser.previous_token()),
                    },
                    name,
                })
            }
            TokenKind::Keyword(KeywordKind::Context) => {
                parser.consume(TokenKind::Keyword(KeywordKind::Context))?;

                Ok(Primitive::Context {
                    info: NodeInfo {
                        id: parser.assign_id(),
                        span: Span::from(parser.previous_token()),
                    },
                })
            }
            TokenKind::LeftParen => {
                if let Some(closure) = parser.rollbacking(|parser| {
                    let start_span = parser.start_span();
                    let params = parser.punctuated(
                        TokenKind::LeftParen,
                        TokenKind::RightParen,
                        TokenKind::Comma,
                        Primitive::parse,
                    )?;
                    parser.consume(TokenKind::ArrowRight)?;
                    let body = BlockStmt::parse(parser)?;
                    let end_span = parser.end_span();

                    Ok(Primitive::Closure {
                        info: NodeInfo {
                            id: parser.assign_id(),
                            span: Span::from((start_span, end_span)),
                        },
                        params,
                        body,
                    })
                }) {
                    Ok(closure)
                } else {
                    let start_span = parser.start_span();
                    let expr =
                        parser.enclosed(TokenKind::LeftParen, TokenKind::RightParen, Expr::parse)?;
                    let end_span = parser.end_span();

                    Ok(Primitive::Grouping {
                        info: NodeInfo {
                            id: parser.assign_id(),
                            span: Span::from((start_span, end_span)),
                        },
                        expr: Box::new(expr),
                    })
                }
            }
            TokenKind::LeftBrace => {
                let start_span = parser.start_span();
                let object = ObjectDecl::parse(parser)?;
                let end_span = parser.end_span();

                Ok(Primitive::Object {
                    info: NodeInfo {
                        id: parser.assign_id(),
                        span: Span::from((start_span, end_span)),
                    },
                    value: object,
                })
            }
            _ => Err(Error::new(
                CompileErrorKind::UnrecognizedPrimitive {
                    primitive: parser.peek().to_string(),
                },
                parser.peek_token(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{ast::expr::primitive::Primitive, Parser};

    #[test]
    fn parses_basic_primitives() {
        let primitive =
            Parser::parse_ast_str::<Primitive>("foo").expect("Failed to parse primitive");
        assert_debug_snapshot!("basic_identifier", primitive);

        let primitive =
            Parser::parse_ast_str::<Primitive>("@foo").expect("Failed to parse primitive");
        assert_debug_snapshot!("basic_story_identifier", primitive);

        let primitive =
            Parser::parse_ast_str::<Primitive>("(x)").expect("Failed to parse primitive");
        assert_debug_snapshot!("basic_grouping", primitive);

        let primitive =
            Parser::parse_ast_str::<Primitive>("context").expect("Failed to parse primitive");
        assert_debug_snapshot!("basic_context", primitive);
    }

    #[test]
    fn parses_object_primitive() {
        let primitive = Parser::parse_ast_str::<Primitive>("{ key1: 42, key2: true }")
            .expect("Failed to parse primitive");

        assert_debug_snapshot!(primitive);
    }

    #[test]
    fn parses_closure_primitive() {
        let primitive = Parser::parse_ast_str::<Primitive>("(x, y) => { x + y; }")
            .expect("Failed to parse primitive");

        assert_debug_snapshot!(primitive);
    }
}
