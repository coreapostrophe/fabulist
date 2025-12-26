use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{expect_token, Parsable};

#[derive(Debug, PartialEq)]
pub struct ModuleStmt {
    pub path: String,
    pub alias: Option<String>,
}

impl Parsable for ModuleStmt {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
        parser.consume(TokenKind::Keyword(KeywordKind::Module))?;

        let path = expect_token!(parser, TokenKind::String, "module string path")?;

        let alias = if parser.r#match(&[TokenKind::Keyword(KeywordKind::As)]) {
            Some(expect_token!(
                parser,
                TokenKind::Identifier,
                "module alias"
            )?)
        } else {
            None
        };

        parser.consume(TokenKind::Semicolon)?;

        Ok(ModuleStmt { path, alias })
    }
}

#[cfg(test)]
mod module_stmt_tests {
    use fabc_lexer::Lexer;

    use crate::{ast::stmt::module::ModuleStmt, Parser};

    #[test]
    fn parses_module_stmt_without_alias() {
        let source = r#"module "my/module/path";"#;
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");
        let module_stmt =
            Parser::parse::<ModuleStmt>(&tokens).expect("Failed to parse module statement");

        let expected = ModuleStmt {
            path: "my/module/path".to_string(),
            alias: None,
        };

        assert_eq!(module_stmt, expected);
    }

    #[test]
    fn parses_module_stmt_with_alias() {
        let source = r#"module "my/module/path" as my_alias;"#;
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");
        let module_stmt =
            Parser::parse::<ModuleStmt>(&tokens).expect("Failed to parse module statement");

        let expected = ModuleStmt {
            path: "my/module/path".to_string(),
            alias: Some("my_alias".to_string()),
        };

        assert_eq!(module_stmt, expected);
    }
}
