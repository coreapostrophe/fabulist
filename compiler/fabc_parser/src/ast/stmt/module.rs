use fabc_lexer::{keywords::KeywordKind, tokens::Token};

use crate::{error::Error, Parsable};

#[derive(Debug, PartialEq)]
pub struct ModuleStmt {
    pub path: String,
    pub alias: Option<String>,
}

impl Parsable for ModuleStmt {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
        parser.consume(Token::Keyword(KeywordKind::Module))?;

        let path = if let Token::String(path) = parser.advance() {
            path.to_string()
        } else {
            return Err(Error::ExpectedFound {
                expected: "path string".to_string(),
                found: parser.peek().to_string(),
            });
        };

        if parser.r#match(vec![Token::Keyword(KeywordKind::As)]) {
            let alias = if let Token::Identifier(alias) = parser.advance() {
                alias.to_string()
            } else {
                return Err(Error::ExpectedFound {
                    expected: "identifier".to_string(),
                    found: parser.peek().to_string(),
                });
            };

            Ok(ModuleStmt {
                path: path.to_string(),
                alias: Some(alias.to_string()),
            })
        } else {
            Ok(ModuleStmt {
                path: path.to_string(),
                alias: None,
            })
        }
    }
}

#[cfg(test)]
mod module_stmt_tests {
    use fabc_lexer::Lexer;

    use crate::{ast::stmt::module::ModuleStmt, Parsable};

    #[test]
    fn parses_module_stmt_without_alias() {
        let source = r#"module "my/module/path""#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source code");

        let mut parser = crate::Parser::new(tokens);
        let module_stmt = ModuleStmt::parse(&mut parser).expect("Failed to parse module statement");

        let expected = ModuleStmt {
            path: "my/module/path".to_string(),
            alias: None,
        };

        assert_eq!(module_stmt, expected);
    }

    #[test]
    fn parses_module_stmt_with_alias() {
        let source = r#"module "my/module/path" as my_alias"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source code");

        let mut parser = crate::Parser::new(tokens);
        let module_stmt = ModuleStmt::parse(&mut parser).expect("Failed to parse module statement");

        let expected = ModuleStmt {
            path: "my/module/path".to_string(),
            alias: Some("my_alias".to_string()),
        };

        assert_eq!(module_stmt, expected);
    }
}
