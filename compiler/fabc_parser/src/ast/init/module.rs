use fabc_error::{Error, Span};
use fabc_lexer::{keywords::KeywordKind, tokens::TokenKind};

use crate::{ast::NodeInfo, expect_token, Parsable, Parser};

#[derive(Debug, PartialEq)]
pub struct ModuleInit {
    pub info: NodeInfo,
    pub path: String,
    pub alias: Option<String>,
}

impl Parsable for ModuleInit {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let start_span = parser.start_span();
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
        let end_span = parser.end_span();

        Ok(ModuleInit {
            info: NodeInfo {
                id: parser.assign_id(),
                span: Span::from((start_span, end_span)),
            },
            path,
            alias,
        })
    }
}

#[cfg(test)]
mod tests {
    use fabc_error::{LineCol, Span};
    use fabc_lexer::Lexer;

    use crate::{
        ast::{init::module::ModuleInit, NodeInfo},
        Parser,
    };

    #[test]
    fn parses_module_init_without_alias() {
        let source = r#"module "my/module/path";"#;
        let tokens = Lexer::tokenize(source);
        let module_init =
            Parser::parse_ast::<ModuleInit>(&tokens).expect("Failed to parse module init");
        let expected = ModuleInit {
            info: NodeInfo {
                id: 0,
                span: Span::from((LineCol::new(1, 1), LineCol::new(1, 24))),
            },
            path: "my/module/path".to_string(),
            alias: None,
        };

        assert_eq!(module_init, expected);
    }

    #[test]
    fn parses_module_init_with_alias() {
        let source = r#"module "my/module/path" as my_alias;"#;
        let tokens = Lexer::tokenize(source);
        let module_init =
            Parser::parse_ast::<ModuleInit>(&tokens).expect("Failed to parse module init");

        let expected = ModuleInit {
            info: NodeInfo {
                id: 0,
                span: Span::from((LineCol::new(1, 1), LineCol::new(1, 36))),
            },
            path: "my/module/path".to_string(),
            alias: Some("my_alias".to_string()),
        };

        assert_eq!(module_init, expected);
    }
}
