use fabc_error::{Error, Span};
use fabc_lexer::tokens::TokenKind;

use crate::{
    ast::{decl::quote::QuoteDecl, NodeInfo},
    expect_token, Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub struct DialogueElement {
    pub info: NodeInfo,
    pub speaker: String,
    pub quotes: Vec<QuoteDecl>,
}

impl Parsable for DialogueElement {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let start_span = parser.start_span();
        let speaker =
            parser.enclosed(TokenKind::LeftBracket, TokenKind::RightBracket, |parser| {
                expect_token!(parser, TokenKind::Identifier, "speaker identifier")
            })?;

        let mut quotes = Vec::new();
        while parser.r#match(&[TokenKind::Greater]) {
            let quote = QuoteDecl::parse(parser)?;
            quotes.push(quote);
        }

        let end_span = parser.end_span();

        Ok(DialogueElement {
            info: NodeInfo {
                id: parser.assign_id(),
                span: Span::from((start_span, end_span)),
            },
            speaker,
            quotes,
        })
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{ast::init::story::part::element::dialogue::DialogueElement, Parser};

    #[test]
    fn parses_dialogue_element() {
        let dialogue = Parser::parse_ast_str::<DialogueElement>(
            r#"
            [narrator]
            > "Hello there!" { emotion: "happy", volume: 5 }
            > "How are you?" { emotion: "curious" }
        "#,
        )
        .expect("Failed to parse dialogue");

        assert_debug_snapshot!(dialogue);
    }
}
