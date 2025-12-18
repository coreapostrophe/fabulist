use fabc_lexer::{keywords::KeywordKind, tokens::Token};

use crate::{
    ast::{
        expr::{Expr, Primary},
        literal::Literal,
        primitive::Primitive,
        stmt::{ElseClause, Stmt},
    },
    error::Error,
};

pub mod ast;
pub mod error;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Stmt, Error> {
        self.statement()
    }

    fn r#match(&mut self, expected: Vec<Token>) -> bool {
        if self.is_at_end() {
            return false;
        }
        if expected.contains(self.peek()) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn consume(&mut self, expected: Token, error: Error) -> Result<&Token, Error> {
        if self.peek() == &expected {
            Ok(self.advance())
        } else {
            Err(error)
        }
    }

    fn _synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if let Token::Semicolon = self.previous() {
                return;
            }

            if let Token::Keyword(
                KeywordKind::Let
                | KeywordKind::Fn
                | KeywordKind::For
                | KeywordKind::If
                | KeywordKind::While
                | KeywordKind::Return,
            ) = self.peek()
            {
                return;
            }

            self.advance();
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn statement(&mut self) -> Result<Stmt, Error> {
        if self.is_at_end() {
            return Err(Error::UnexpectedEndOfInput);
        }

        match self.peek() {
            Token::Keyword(KeywordKind::Fn) => self.function_statement(),
            Token::Keyword(KeywordKind::Goto) => self.goto_statement(),
            Token::Keyword(KeywordKind::If) => self.if_statement(),
            Token::LeftBrace => self.block_statement(),
            Token::Keyword(KeywordKind::Let) => self.let_statement(),
            _ => self.expr_statement(),
        }
    }

    fn function_statement(&mut self) -> Result<Stmt, Error> {
        self.advance();

        let name = if let Token::Identifier(ident) = self.peek() {
            ident.clone()
        } else {
            return Err(Error::ExpectedFound(
                "function name".to_string(),
                self.peek().to_string(),
            ));
        };

        self.advance();

        self.consume(
            Token::LeftParen,
            Error::ExpectedFound("(".to_string(), self.peek().to_string()),
        )?;

        let mut parameters = Vec::new();
        if self.peek() != &Token::RightParen {
            loop {
                if let Token::Identifier(param_name) = self.peek() {
                    parameters.push(param_name.clone());
                } else {
                    return Err(Error::ExpectedFound(
                        "parameter name".to_string(),
                        self.peek().to_string(),
                    ));
                }
                self.advance();

                if !self.r#match(vec![Token::Comma]) {
                    break;
                }
            }
        }

        self.consume(
            Token::RightParen,
            Error::ExpectedFound(")".to_string(), self.peek().to_string()),
        )?;

        let body = Box::new(self.statement()?);

        Ok(Stmt::Function {
            name,
            parameters,
            body,
        })
    }

    fn goto_statement(&mut self) -> Result<Stmt, Error> {
        self.advance();

        let path = if let Token::Path(segments) = self.peek() {
            segments.clone()
        } else {
            return Err(Error::ExpectedFound(
                "path".to_string(),
                self.peek().to_string(),
            ));
        };

        self.advance();

        self.consume(Token::Semicolon, Error::MissingSemicolon)?;

        Ok(Stmt::Goto {
            path: Primitive::Path(path),
        })
    }

    fn if_statement(&mut self) -> Result<Stmt, Error> {
        self.advance();

        let condition = self.expression()?;

        let then_branch = Box::new(self.statement()?);

        let else_branch = if self.r#match(vec![Token::Keyword(KeywordKind::Else)]) {
            if self.r#match(vec![Token::Keyword(KeywordKind::If)]) {
                Some(ElseClause::If(Box::new(self.if_statement()?)))
            } else {
                Some(ElseClause::Block(Box::new(self.statement()?)))
            }
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn block_statement(&mut self) -> Result<Stmt, Error> {
        let mut statements = Vec::new();

        self.consume(
            Token::LeftBrace,
            Error::ExpectedFound("{".to_string(), self.peek().to_string()),
        )?;

        while !self.is_at_end() && self.peek() != &Token::RightBrace {
            let stmt = self.statement()?;
            statements.push(stmt);
        }

        self.consume(
            Token::RightBrace,
            Error::ExpectedFound("}".to_string(), self.peek().to_string()),
        )?;

        Ok(Stmt::Block(statements))
    }

    fn expr_statement(&mut self) -> Result<Stmt, Error> {
        let expr = self.expression()?;
        self.consume(Token::Semicolon, Error::MissingSemicolon)?;
        Ok(Stmt::Expr(expr))
    }

    fn let_statement(&mut self) -> Result<Stmt, Error> {
        self.advance();

        let name = if let Token::Identifier(ident) = self.peek() {
            ident.clone()
        } else {
            return Err(Error::ExpectedFound(
                "identifier".to_string(),
                self.peek().to_string(),
            ));
        };

        self.advance();

        self.consume(
            Token::Equal,
            Error::ExpectedFound("=".to_string(), self.peek().to_string()),
        )?;

        let initializer = self.expression()?;

        self.consume(Token::Semicolon, Error::MissingSemicolon)?;

        Ok(Stmt::Let { name, initializer })
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;

        while self.r#match(vec![Token::BangEqual, Token::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.term()?;

        while self.r#match(vec![
            Token::Greater,
            Token::GreaterEqual,
            Token::Less,
            Token::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.factor()?;

        while self.r#match(vec![Token::Minus, Token::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;

        while self.r#match(vec![Token::Slash, Token::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.r#match(vec![Token::Bang, Token::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        if self.is_at_end() {
            return Err(Error::UnexpectedEndOfInput);
        }

        match self.peek() {
            Token::Keyword(keyword_kind) => match keyword_kind {
                KeywordKind::True => {
                    self.advance();
                    Ok(Expr::Primary(Primary::Literal(Literal::Boolean(true))))
                }
                KeywordKind::False => {
                    self.advance();
                    Ok(Expr::Primary(Primary::Literal(Literal::Boolean(false))))
                }
                KeywordKind::None => {
                    self.advance();
                    Ok(Expr::Primary(Primary::Literal(Literal::None)))
                }
                _ => Err(Error::UnhandledKeywordLiteral),
            },
            Token::String(string) => {
                let value = string.clone();
                self.advance();
                Ok(Expr::Primary(Primary::Literal(Literal::String(value))))
            }
            Token::Number(number) => {
                let value = *number;
                self.advance();
                Ok(Expr::Primary(Primary::Literal(Literal::Number(value))))
            }
            Token::Identifier(identifier) => {
                let name = identifier.clone();
                self.advance();
                Ok(Expr::Primary(Primary::Primitive(Primitive::Identifier(
                    name,
                ))))
            }
            Token::Path(path) => {
                let segments = path.clone();
                self.advance();
                Ok(Expr::Primary(Primary::Primitive(Primitive::Path(segments))))
            }
            Token::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(
                    Token::RightParen,
                    Error::ExpectedFound(")".to_string(), self.peek().to_string()),
                )?;
                Ok(Expr::Grouping(Box::new(expr)))
            }
            _ => Err(Error::UnhandledPrimaryExpression),
        }
    }
}

#[cfg(test)]
mod parser_tests {
    use fabc_lexer::{tokens::Token, Lexer};

    use crate::ast::{
        expr::{Expr, Primary},
        literal::Literal,
    };

    use super::Parser;

    #[test]
    fn parses_let_statements() {
        let source = "let x = 42;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
        let stmt = parser.statement().expect("Failed to parse");

        let expected = crate::ast::stmt::Stmt::Let {
            name: "x".to_string(),
            initializer: Expr::Primary(Primary::Literal(Literal::Number(42.0))),
        };

        assert_eq!(stmt, expected);
    }

    #[test]
    fn parses_expression_statements() {
        let source = "x + 1;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
        let stmt = parser.statement().expect("Failed to parse");

        let expected = crate::ast::stmt::Stmt::Expr(Expr::Binary {
            left: Box::new(Expr::Primary(Primary::Primitive(
                crate::ast::primitive::Primitive::Identifier("x".to_string()),
            ))),
            operator: Token::Plus,
            right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(1.0)))),
        });

        assert_eq!(stmt, expected);
    }

    #[test]
    fn parses_goto_statements() {
        let source = "goto my::module::label;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
        let stmt = parser.statement().expect("Failed to parse");

        let expected = crate::ast::stmt::Stmt::Goto {
            path: crate::ast::primitive::Primitive::Path(vec![
                "my".to_string(),
                "module".to_string(),
                "label".to_string(),
            ]),
        };

        assert_eq!(stmt, expected);
    }

    #[test]
    fn parses_fn_statements() {
        let source = "fn myFunction(param1, param2) { let x = 10; }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
        let stmt = parser.statement().expect("Failed to parse");

        let expected = crate::ast::stmt::Stmt::Function {
            name: "myFunction".to_string(),
            parameters: vec!["param1".to_string(), "param2".to_string()],
            body: Box::new(crate::ast::stmt::Stmt::Block(vec![
                crate::ast::stmt::Stmt::Let {
                    name: "x".to_string(),
                    initializer: Expr::Primary(Primary::Literal(Literal::Number(10.0))),
                },
            ])),
        };

        assert_eq!(stmt, expected);
    }

    #[test]
    fn parses_if_statements() {
        let source = "if true { let y = 10; } else { let y = 20; }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
        let stmt = parser.statement().expect("Failed to parse");

        let expected = crate::ast::stmt::Stmt::If {
            condition: Expr::Primary(Primary::Literal(Literal::Boolean(true))),
            then_branch: Box::new(crate::ast::stmt::Stmt::Block(vec![
                crate::ast::stmt::Stmt::Let {
                    name: "y".to_string(),
                    initializer: Expr::Primary(Primary::Literal(Literal::Number(10.0))),
                },
            ])),
            else_branch: Some(crate::ast::stmt::ElseClause::Block(Box::new(
                crate::ast::stmt::Stmt::Block(vec![crate::ast::stmt::Stmt::Let {
                    name: "y".to_string(),
                    initializer: Expr::Primary(Primary::Literal(Literal::Number(20.0))),
                }]),
            ))),
        };

        assert_eq!(stmt, expected);
    }

    #[test]
    fn parses_block_statements() {
        let source = "{ let a = 1; let b = 2; }";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
        let stmt = parser.statement().expect("Failed to parse");

        let expected = crate::ast::stmt::Stmt::Block(vec![
            crate::ast::stmt::Stmt::Let {
                name: "a".to_string(),
                initializer: Expr::Primary(Primary::Literal(Literal::Number(1.0))),
            },
            crate::ast::stmt::Stmt::Let {
                name: "b".to_string(),
                initializer: Expr::Primary(Primary::Literal(Literal::Number(2.0))),
            },
        ]);

        assert_eq!(stmt, expected);
    }

    #[test]
    fn parses_binary_expression() {
        let source = "1 + 2 * 3;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
        let expr = parser.expression().expect("Failed to parse expression");

        let expected = Expr::Binary {
            left: Box::new(Expr::Primary(Primary::Literal(Literal::Number(1.0)))),
            operator: Token::Plus,
            right: Box::new(Expr::Binary {
                left: Box::new(Expr::Primary(Primary::Literal(Literal::Number(2.0)))),
                operator: Token::Star,
                right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(3.0)))),
            }),
        };

        assert_eq!(expr, expected);
    }

    #[test]
    fn parses_unary_expression() {
        let source = "-42;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
        let expr = parser.expression().expect("Failed to parse expression");

        let expected = Expr::Unary {
            operator: Token::Minus,
            right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(42.0)))),
        };

        assert_eq!(expr, expected);
    }

    #[test]
    fn parses_primary_expression() {
        let source = "true";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
        let expr = parser.expression().expect("Failed to parse expression");

        let expected = Expr::Primary(Primary::Literal(Literal::Boolean(true)));

        assert_eq!(expr, expected);
    }

    #[test]
    fn parses_grouping_expression() {
        let source = "(1 + 2) * 3;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
        let expr = parser.expression().expect("Failed to parse expression");

        let expected = Expr::Binary {
            left: Box::new(Expr::Grouping(Box::new(Expr::Binary {
                left: Box::new(Expr::Primary(Primary::Literal(Literal::Number(1.0)))),
                operator: Token::Plus,
                right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(2.0)))),
            }))),
            operator: Token::Star,
            right: Box::new(Expr::Primary(Primary::Literal(Literal::Number(3.0)))),
        };

        assert_eq!(expr, expected);
    }
}
