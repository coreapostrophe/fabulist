use fabc_error::{Error, Span};
use fabc_lexer::tokens::TokenKind;

use crate::{
    ast::{decl::quote::QuoteDecl, NodeInfo},
    Parsable, Parser,
};

#[derive(Debug, PartialEq)]
pub struct SelectionElement {
    pub info: NodeInfo,
    pub choices: Vec<QuoteDecl>,
}

impl Parsable for SelectionElement {
    fn parse(parser: &mut Parser<'_, '_>) -> Result<Self, Error> {
        let start_span = parser.start_span();
        let mut choices = Vec::new();
        while parser.r#match(&[TokenKind::Minus]) {
            let choice = QuoteDecl::parse(parser)?;
            choices.push(choice);
        }
        let end_span = parser.end_span();

        Ok(SelectionElement {
            info: NodeInfo {
                id: parser.assign_id(),
                span: Span::from((start_span, end_span)),
            },
            choices,
        })
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use crate::{ast::init::story::part::element::selection::SelectionElement, Parser};

    #[test]
    fn parses_selection_with_multiple_choices() {
        let selection = Parser::parse_ast_str::<SelectionElement>(
            r#"
            - "Go left." { score: 10, health: 5 }
            - "Go right." { score: 5 }
        "#,
        )
        .expect("Failed to parse selection");

        assert_debug_snapshot!(selection);
    }
}
