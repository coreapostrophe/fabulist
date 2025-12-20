use std::collections::HashMap;

use fabc_lexer::{keywords::KeywordKind, tokens::Token};

use crate::{
    ast::{decl::object::ObjectDecl, expr::Expr},
    Parsable,
};

#[derive(Debug, PartialEq)]
pub struct Metadata {
    pub map: HashMap<String, Expr>,
}

impl Parsable for Metadata {
    fn parse(parser: &mut crate::Parser) -> Result<Self, crate::error::Error> {
        parser.consume(Token::Keyword(KeywordKind::Story))?;

        let map = ObjectDecl::parse(parser)?.map;

        Ok(Metadata { map })
    }
}

#[cfg(test)]
mod metadata_tests {
    use std::collections::HashMap;

    use fabc_lexer::Lexer;

    use crate::{
        ast::{
            expr::{literal::Literal, Expr, Primary},
            story::metadata::Metadata,
        },
        Parsable, Parser,
    };

    #[test]
    fn parses_metadat() {
        let source = r#"
            Story {
                title: "My Story",
            }
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.tokenize().expect("Failed to tokenize source");
        let mut parser = Parser::new(&tokens);

        let metadata = Metadata::parse(&mut parser).expect("Failed to parse metadata");

        let expected_map = {
            let mut map = HashMap::new();
            map.insert(
                "title".to_string(),
                Expr::Primary(Primary::Literal(Literal::String("My Story".to_string()))),
            );
            map
        };

        assert_eq!(metadata.map, expected_map);
    }
}
