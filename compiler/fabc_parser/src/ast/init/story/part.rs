use fabc_error::{Error, Span};
use fabc_lexer::tokens::TokenKind;

use crate::{
    ast::{init::story::part::element::Element, NodeInfo},
    expect_token, Parsable, Parser,
};

pub mod element;

#[derive(Debug, PartialEq)]
pub struct Part {
    pub info: NodeInfo,
    pub ident: String,
    pub elements: Vec<Element>,
}

impl Part {
    pub const SYNC_DELIMITERS: &[TokenKind<'_>] = &[TokenKind::Pound];
}

impl Parsable for Part {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let start_span = parser.start_span();
        parser.consume(TokenKind::Pound)?;
        let ident = expect_token!(parser, TokenKind::Identifier, "identifier")?;
        let elements =
            parser.invariant_parse(Element::SYNC_DELIMITERS, Part::SYNC_DELIMITERS, false);
        let end_span = parser.end_span();

        Ok(Part {
            info: NodeInfo {
                id: parser.assign_id(),
                span: Span::from((start_span, end_span)),
            },
            ident,
            elements,
        })
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{ast::init::story::part::Part, Parser};

    #[test]
    fn parses_part() {
        let part = Parser::parse_ast_str::<Part>(
            r##"
            # intro
            * "This is a narration."
        "##,
        )
        .expect("Failed to parse part");

        assert_debug_snapshot!(part);
    }
}
