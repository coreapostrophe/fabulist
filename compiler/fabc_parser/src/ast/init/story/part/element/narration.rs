use fabc_error::{Error, Span};
use fabc_lexer::tokens::TokenKind;

use crate::{
    ast::{decl::quote::QuoteDecl, NodeInfo},
    Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub struct NarrationElement {
    pub info: NodeInfo,
    pub quote: QuoteDecl,
}

impl Parsable for NarrationElement {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let start_span = parser.start_span();
        parser.consume(TokenKind::Asterisk)?;
        let quote = QuoteDecl::parse(parser)?;
        let end_span = parser.end_span();

        Ok(NarrationElement {
            info: NodeInfo {
                id: parser.assign_id(),
                span: Span::from((start_span, end_span)),
            },
            quote,
        })
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{ast::init::story::part::element::narration::NarrationElement, Parser};

    #[test]
    fn parses_narration_without_properties() {
        let narration = Parser::parse_ast_str::<NarrationElement>("* \"This is a narration.\"")
            .expect("Failed to parse narration");

        assert_debug_snapshot!(narration);
    }

    #[test]
    fn parses_narration_with_properties() {
        let narration = Parser::parse_ast_str::<NarrationElement>(
            "* \"This is a narration.\" { mood: happy, volume: loud }",
        )
        .expect("Failed to parse narration");

        assert_debug_snapshot!(narration);
    }
}
