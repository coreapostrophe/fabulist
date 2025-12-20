use fabc_lexer::{keywords::KeywordKind, tokens::Token};

use crate::{expect_token, Parsable};

#[derive(Debug, PartialEq)]
pub struct ModuleStmt {
    pub path: String,
    pub alias: Option<String>,
}

impl Parsable for ModuleStmt {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
        parser.consume(Token::Keyword(KeywordKind::Module))?;

        let path = expect_token!(parser, Token::String, "module string path")?;

        let alias = if parser.r#match(&[Token::Keyword(KeywordKind::As)]) {
            Some(expect_token!(parser, Token::Identifier, "module alias")?)
        } else {
            None
        };

        parser.consume(Token::Semicolon)?;

        Ok(ModuleStmt { path, alias })
    }
}

#[cfg(test)]
mod module_stmt_tests {
    use fabc_lexer::Lexer;

    use crate::{ast::stmt::module::ModuleStmt, Parsable};

    #[test]
    fn parses_module_stmt_without_alias() {
        let source = r#"module "my/module/path";"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source code");

        let mut parser = crate::Parser::new(&tokens);
        let module_stmt = ModuleStmt::parse(&mut parser).expect("Failed to parse module statement");

        let expected = ModuleStmt {
            path: "my/module/path".to_string(),
            alias: None,
        };

        assert_eq!(module_stmt, expected);
    }

    #[test]
    fn parses_module_stmt_with_alias() {
        let source = r#"module "my/module/path" as my_alias;"#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source code");

        let mut parser = crate::Parser::new(&tokens);
        let module_stmt = ModuleStmt::parse(&mut parser).expect("Failed to parse module statement");

        let expected = ModuleStmt {
            path: "my/module/path".to_string(),
            alias: Some("my_alias".to_string()),
        };

        assert_eq!(module_stmt, expected);
    }
}
