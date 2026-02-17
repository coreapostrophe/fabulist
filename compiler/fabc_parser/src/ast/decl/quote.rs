use fabc_error::{Error, Span};
use fabc_lexer::tokens::TokenKind;

use crate::{
    ast::{decl::object::ObjectDecl, NodeInfo},
    expect_token, Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub struct QuoteDecl {
    pub info: NodeInfo,
    pub text: String,
    pub properties: Option<ObjectDecl>,
}

impl Parsable for QuoteDecl {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let start_span = parser.start_span();

        let text = expect_token!(parser, TokenKind::String, "quote text")?;
        let properties = if parser.peek() == &TokenKind::LeftBrace {
            Some(ObjectDecl::parse(parser)?)
        } else {
            None
        };

        let end_span = parser.end_span();

        Ok(QuoteDecl {
            info: NodeInfo {
                id: parser.assign_id(),
                span: Span::from((start_span, end_span)),
            },
            text,
            properties,
        })
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{ast::decl::quote::QuoteDecl, Parser};

    #[test]
    fn parses_quote_decl_without_properties() {
        let quote_decl = Parser::parse_ast_str::<QuoteDecl>("\"This is a quote.\"")
            .expect("Failed to parse quote");

        assert_debug_snapshot!(quote_decl);
    }

    #[test]
    fn parses_quote_decl_with_properties() {
        let quote_decl = Parser::parse_ast_str::<QuoteDecl>(
            "\"This is a quote with properties.\" { author: \"Alice\", length: 30 }",
        )
        .expect("Failed to parse quote");

        assert_debug_snapshot!(quote_decl);
    }
}
