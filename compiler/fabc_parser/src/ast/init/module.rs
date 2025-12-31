use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{expect_token, Parsable};

#[derive(Debug, PartialEq)]
pub struct ModuleInit {
    pub id: usize,
    pub path: String,
    pub alias: Option<String>,
}

impl Parsable for ModuleInit {
    fn parse<'src, 'tok>(
        parser: &mut crate::Parser<'src, 'tok>,
    ) -> Result<Self, crate::error::Error> {
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

        Ok(ModuleInit {
            id: parser.assign_id(),
            path,
            alias,
        })
    }
}

#[cfg(test)]
mod module_stmt_tests {
    use fabc_lexer::Lexer;

    use crate::{ast::init::module::ModuleInit, Parser};

    #[test]
    fn parses_module_init_without_alias() {
        let source = r#"module "my/module/path";"#;
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");
        let module_init =
            Parser::parse_ast::<ModuleInit>(&tokens).expect("Failed to parse module init");
        let expected = ModuleInit {
            id: 0,
            path: "my/module/path".to_string(),
            alias: None,
        };

        assert_eq!(module_init, expected);
    }

    #[test]
    fn parses_module_init_with_alias() {
        let source = r#"module "my/module/path" as my_alias;"#;
        let tokens = Lexer::tokenize(source).expect("Failed to tokenize source code");
        let module_init =
            Parser::parse_ast::<ModuleInit>(&tokens).expect("Failed to parse module init");

        let expected = ModuleInit {
            id: 0,
            path: "my/module/path".to_string(),
            alias: Some("my_alias".to_string()),
        };

        assert_eq!(module_init, expected);
    }
}
