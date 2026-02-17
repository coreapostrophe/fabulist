use fabc_error::{kind::CompileErrorKind, Error, LineCol, Span};
use fabc_lexer::{
    keywords::KeywordKind,
    tokens::{Token, TokenKind},
};

use crate::{
    ast::{
        expr::{literal::Literal, primitive::Primitive},
        NodeInfo,
    },
    Parsable, Parser,
};

pub mod literal;
pub mod primitive;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinaryOperator {
    EqualEqual,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Add,
    Subtraction,
    Multiply,
    Divide,
    And,
    Or,
}

impl TryFrom<&Token<'_>> for BinaryOperator {
    type Error = Error;

    fn try_from(token: &Token<'_>) -> Result<Self, Self::Error> {
        match token.kind {
            TokenKind::EqualEqual => Ok(BinaryOperator::EqualEqual),
            TokenKind::BangEqual => Ok(BinaryOperator::NotEqual),
            TokenKind::Greater => Ok(BinaryOperator::Greater),
            TokenKind::GreaterEqual => Ok(BinaryOperator::GreaterEqual),
            TokenKind::Less => Ok(BinaryOperator::Less),
            TokenKind::LessEqual => Ok(BinaryOperator::LessEqual),
            TokenKind::Plus => Ok(BinaryOperator::Add),
            TokenKind::Minus => Ok(BinaryOperator::Subtraction),
            TokenKind::Asterisk => Ok(BinaryOperator::Multiply),
            TokenKind::Slash => Ok(BinaryOperator::Divide),
            _ => Err(Error::new(
                CompileErrorKind::InvalidOperator {
                    operator: token.kind.to_string(),
                },
                token,
            )),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
    Negate,
}

impl TryFrom<&Token<'_>> for UnaryOperator {
    type Error = Error;

    fn try_from(token: &Token<'_>) -> Result<Self, Self::Error> {
        match token.kind {
            TokenKind::Bang => Ok(UnaryOperator::Not),
            TokenKind::Minus => Ok(UnaryOperator::Negate),
            _ => Err(Error::new(
                CompileErrorKind::InvalidOperator {
                    operator: token.kind.to_string(),
                },
                token,
            )),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LogicalOperator {
    And,
    Or,
}

impl TryFrom<&Token<'_>> for LogicalOperator {
    type Error = Error;

    fn try_from(token: &Token<'_>) -> Result<Self, Self::Error> {
        match token.kind {
            TokenKind::Keyword(KeywordKind::And) => Ok(LogicalOperator::And),
            TokenKind::Keyword(KeywordKind::Or) => Ok(LogicalOperator::Or),
            _ => Err(Error::new(
                CompileErrorKind::InvalidOperator {
                    operator: token.kind.to_string(),
                },
                token,
            )),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Primary {
    Literal(Literal),
    Primitive(Primitive),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Binary {
        info: NodeInfo,
        left: Box<Expr>,
        operator: BinaryOperator,
        right: Box<Expr>,
    },
    Unary {
        info: NodeInfo,
        operator: UnaryOperator,
        right: Box<Expr>,
    },
    Assignment {
        info: NodeInfo,
        name: Box<Expr>,
        value: Box<Expr>,
    },
    MemberAccess {
        info: NodeInfo,
        left: Box<Expr>,
        members: Vec<Expr>,
    },
    Call {
        info: NodeInfo,
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
    Primary {
        info: NodeInfo,
        value: Primary,
    },
    Grouping {
        info: NodeInfo,
        expression: Box<Expr>,
    },
}

impl Expr {
    pub fn info(&self) -> &NodeInfo {
        match self {
            Expr::Binary { info, .. } => info,
            Expr::Unary { info, .. } => info,
            Expr::Assignment { info, .. } => info,
            Expr::MemberAccess { info, .. } => info,
            Expr::Call { info, .. } => info,
            Expr::Primary { info, .. } => info,
            Expr::Grouping { info, .. } => info,
        }
    }
}

impl Expr {
    pub fn assignment(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let start_span = parser.start_span();
        let mut expr = Self::logical(parser)?;

        if parser.r#match(&[TokenKind::Equal]) {
            let value = Self::assignment(parser)?;
            let end_span = parser.end_span();

            expr = Expr::Assignment {
                info: NodeInfo {
                    id: parser.assign_id(),
                    span: Span::from((start_span, end_span)),
                },
                name: Box::new(expr),
                value: Box::new(value),
            }
        }

        Ok(expr)
    }

    fn logical(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let start_span = parser.start_span();
        let mut expr = Self::equality(parser)?;

        while parser.r#match(&[
            TokenKind::Keyword(KeywordKind::And),
            TokenKind::Keyword(KeywordKind::Or),
        ]) {
            let operator = LogicalOperator::try_from(parser.previous_token())?;
            let right = Self::equality(parser)?;
            let end_span = parser.end_span();

            expr = Expr::Binary {
                info: NodeInfo {
                    id: parser.assign_id(),
                    span: Span::from((start_span, end_span)),
                },
                left: Box::new(expr),
                operator: match operator {
                    LogicalOperator::And => BinaryOperator::And,
                    LogicalOperator::Or => BinaryOperator::Or,
                },
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let start_span = parser.start_span();
        let mut expr = Self::comparison(parser)?;

        while parser.r#match(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            let operator = BinaryOperator::try_from(parser.previous_token())?;
            let right = Self::comparison(parser)?;
            let end_span = parser.end_span();

            expr = Expr::Binary {
                info: NodeInfo {
                    id: parser.assign_id(),
                    span: Span::from((start_span, end_span)),
                },
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let start_span = parser.start_span();
        let mut expr = Self::term(parser)?;

        while parser.r#match(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let operator = BinaryOperator::try_from(parser.previous_token())?;
            let right = Self::term(parser)?;
            let end_span = parser.end_span();

            expr = Expr::Binary {
                info: NodeInfo {
                    id: parser.assign_id(),
                    span: Span::from((start_span, end_span)),
                },
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let start_span = parser.start_span();
        let mut expr = Self::factor(parser)?;

        while parser.r#match(&[TokenKind::Minus, TokenKind::Plus]) {
            let operator = BinaryOperator::try_from(parser.previous_token())?;
            let right = Self::factor(parser)?;
            let end_span = parser.end_span();

            expr = Expr::Binary {
                info: NodeInfo {
                    id: parser.assign_id(),
                    span: Span::from((start_span, end_span)),
                },
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let start_span = LineCol::from_token(parser.peek_token());
        let mut expr = Self::unary(parser)?;

        while parser.r#match(&[TokenKind::Slash, TokenKind::Asterisk]) {
            let operator = BinaryOperator::try_from(parser.previous_token())?;
            let right = Self::unary(parser)?;
            let end_span = LineCol::from_token(parser.previous_token());

            expr = Expr::Binary {
                info: NodeInfo {
                    id: parser.assign_id(),
                    span: Span::from((start_span, end_span)),
                },
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        if parser.r#match(&[TokenKind::Bang, TokenKind::Minus]) {
            let start_span = LineCol::from_token(parser.peek_token());
            let operator = UnaryOperator::try_from(parser.previous_token())?;
            let right = Self::unary(parser)?;
            let end_span = LineCol::from_token(parser.previous_token());

            return Ok(Expr::Unary {
                info: NodeInfo {
                    id: parser.assign_id(),
                    span: Span::from((start_span, end_span)),
                },
                operator,
                right: Box::new(right),
            });
        }

        Self::member_access(parser)
    }

    fn member_access(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let start_span = parser.start_span();
        let mut expr = Self::call(parser)?;

        if parser.r#match(&[TokenKind::Dot]) {
            let mut members = Vec::new();
            loop {
                let member = Self::call(parser)?;
                members.push(member);

                if !parser.r#match(&[TokenKind::Dot]) {
                    break;
                }
            }
            let end_span = parser.end_span();

            expr = Expr::MemberAccess {
                info: NodeInfo {
                    id: parser.assign_id(),
                    span: Span::from((start_span, end_span)),
                },
                left: Box::new(expr),
                members,
            };
        }

        Ok(expr)
    }

    fn call(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        let start_span = parser.start_span();
        let mut expr = Self::primary(parser)?;

        if parser.peek() == &TokenKind::LeftParen {
            let arguments = parser.punctuated(
                TokenKind::LeftParen,
                TokenKind::RightParen,
                TokenKind::Comma,
                |parser| Expr::parse(parser),
            )?;
            let end_span = parser.end_span();

            expr = Expr::Call {
                info: NodeInfo {
                    id: parser.assign_id(),
                    span: Span::from((start_span, end_span)),
                },
                callee: Box::new(expr),
                arguments,
            };
        }

        Ok(expr)
    }

    fn primary(parser: &mut Parser<'_, '_>) -> Result<Expr, Error> {
        match parser.peek() {
            // Literals
            TokenKind::String(_)
            | TokenKind::Number(_)
            | TokenKind::Keyword(KeywordKind::True | KeywordKind::False | KeywordKind::None) => {
                let literal = Literal::parse(parser)?;

                Ok(Expr::Primary {
                    info: NodeInfo {
                        id: parser.assign_id(),
                        span: Span::from(parser.previous_token()),
                    },
                    value: Primary::Literal(literal),
                })
            }

            // Primitives
            TokenKind::LeftParen
            | TokenKind::LeftBrace
            | TokenKind::Identifier(_)
            | TokenKind::Keyword(KeywordKind::Context) => {
                let primitive = Primitive::parse(parser)?;

                Ok(Expr::Primary {
                    info: NodeInfo {
                        id: parser.assign_id(),
                        span: Span::from(parser.previous_token()),
                    },
                    value: Primary::Primitive(primitive),
                })
            }
            _ => Err(Error::new(
                CompileErrorKind::UnrecognizedPrimary {
                    primary: parser.peek().to_string(),
                },
                parser.peek_token(),
            )),
        }
    }
}

impl Parsable for Expr {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        Self::assignment(parser)
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{ast::expr::Expr, Parser};

    #[test]
    fn parses_arithmetic_binary_expr() {
        let expr =
            Parser::parse_ast_str::<Expr>("1 + 2 * 3 / 4").expect("Failed to parse expression");
        assert_debug_snapshot!(expr);
    }

    #[test]
    fn parses_equality_expr() {
        let expr =
            Parser::parse_ast_str::<Expr>("10 == 20 != 30").expect("Failed to parse expression");
        assert_debug_snapshot!(expr);
    }

    #[test]
    fn parses_comparison_expr() {
        let expr = Parser::parse_ast_str::<Expr>("5 > 3 < 9 >= 2 <= 10")
            .expect("Failed to parse expression");
        assert_debug_snapshot!(expr);
    }

    #[test]
    fn parses_call_expr() {
        let expr =
            Parser::parse_ast_str::<Expr>("func(arg1, arg2)").expect("Failed to parse expression");
        assert_debug_snapshot!(expr);
    }

    #[test]
    fn parses_member_access_expr() {
        let expr =
            Parser::parse_ast_str::<Expr>("obj.prop1.prop2").expect("Failed to parse expression");
        assert_debug_snapshot!(expr);
    }

    #[test]
    fn parses_unary_expr() {
        let expr = Parser::parse_ast_str::<Expr>("-!42").expect("Failed to parse expression");
        assert_debug_snapshot!(expr);
    }

    #[test]
    fn parses_logical_expr() {
        let expr = Parser::parse_ast_str::<Expr>("true and false or true")
            .expect("Failed to parse expression");
        assert_debug_snapshot!(expr);
    }

    #[test]
    fn parses_assignment_expr() {
        let expr =
            Parser::parse_ast_str::<Expr>("x = 10 + 20").expect("Failed to parse expression");
        assert_debug_snapshot!(expr);
    }
}
