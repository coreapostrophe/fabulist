use fabc_lexer::tokens::Token;

use crate::{ast::expr::Expr, Parsable};

#[derive(Debug, PartialEq)]
pub struct ArgumentBodyDecl {
    pub arguments: Vec<Expr>,
}

impl Parsable for ArgumentBodyDecl {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
        parser.consume(Token::LeftParen)?;

        let mut arguments = Vec::new();
        if parser.peek() != &Token::RightParen {
            loop {
                let expr = Expr::parse(parser)?;
                arguments.push(expr);

                if !parser.r#match(vec![Token::Comma]) {
                    break;
                }
            }
        }

        parser.consume(Token::RightParen)?;

        Ok(ArgumentBodyDecl { arguments })
    }
}

#[cfg(test)]
mod argument_body_decl_tests {

    use crate::{
        ast::{
            decl::argument_body::ArgumentBodyDecl,
            expr::{literal::Literal, Expr, Primary},
        },
        Parsable, Parser,
    };
    use fabc_lexer::Lexer;

    #[test]
    fn parses_argument_body_decl() {
        let source = "(42, true, \"hello\")";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
        let argument_body = ArgumentBodyDecl::parse(&mut parser).expect("Failed to parse");

        assert_eq!(
            argument_body,
            ArgumentBodyDecl {
                arguments: vec![
                    Expr::Primary(Primary::Literal(Literal::Number(42.0))),
                    Expr::Primary(Primary::Literal(Literal::Boolean(true))),
                    Expr::Primary(Primary::Literal(Literal::String("hello".to_string()))),
                ],
            }
        );
    }
}
