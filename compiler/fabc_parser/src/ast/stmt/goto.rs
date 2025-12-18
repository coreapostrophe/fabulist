use fabc_lexer::{keywords::KeywordKind, tokens::Token};

use crate::{ast::primitive::Primitive, error::Error, Parsable, Parser};

#[derive(Debug, PartialEq)]
pub struct GotoStmt {
    pub label: Primitive,
}

impl Parsable for GotoStmt {
    fn parse(parser: &mut Parser) -> Result<Self, Error> {
        parser.consume(Token::Keyword(KeywordKind::Goto))?;

        let path = match parser.advance() {
            Token::Path(segments) => Ok(segments.clone()),
            _ => Err(Error::ExpectedFound(
                "path".to_string(),
                parser.peek().to_string(),
            )),
        }?;

        parser.consume(Token::Semicolon)?;

        Ok(GotoStmt {
            label: Primitive::Path(path),
        })
    }
}

#[cfg(test)]
mod goto_stmt_tests {
    use fabc_lexer::Lexer;

    use crate::{
        ast::{primitive::Primitive, stmt::goto::GotoStmt},
        Parsable, Parser,
    };

    #[test]
    fn parses_goto_statements() {
        let source = "goto my::module::label;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize");

        let mut parser = Parser::new(tokens);
        let stmt = GotoStmt::parse(&mut parser).expect("Failed to parse");

        let expected = GotoStmt {
            label: Primitive::Path(vec![
                "my".to_string(),
                "module".to_string(),
                "label".to_string(),
            ]),
        };

        assert_eq!(stmt, expected);
    }
}
