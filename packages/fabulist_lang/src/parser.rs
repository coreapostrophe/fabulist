//! Pest-based parser entry points for Fabulist source code.
#![allow(missing_docs)]
use pest::Parser;

use crate::{
    error::{OwnedSpan, ParsingError},
    parser::ast::story::StoryAst,
};

pub mod ast;

#[derive(pest_derive::Parser)]
#[grammar = "../grammar/fabulist.pest"]
/// Generated parser for the Fabulist grammar.
pub struct GrammarParser;

/// User-facing parser that produces raw pest pairs for the story grammar.
pub struct FabulistParser;

impl FabulistParser {
    /// Parses a Fabulist source string into a [`StoryAst`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fabulist_lang::parser::FabulistParser;
    ///
    /// let source = r##"
    /// story { "start": "part_1" }
    ///
    /// ## part_1
    /// [Hero]
    /// > "Hello"
    /// "##;
    ///
    /// let ast = FabulistParser::parse(source).expect("parse failure");
    /// assert_eq!(ast.parts.len(), 1);
    /// ```
    pub fn parse(
        source: impl Into<String>,
    ) -> Result<ast::story::StoryAst, Box<pest::error::Error<Rule>>> {
        let source = source.into();
        let mut pairs = GrammarParser::parse(Rule::story, &source).map_err(Box::new)?;

        let story_pair = pairs.next().ok_or_else(|| {
            let owned_span = OwnedSpan {
                input: source.to_string(),
                start: 0,
                end: source.len(),
            };
            Box::new(ParsingError::map_custom_error(
                owned_span,
                "Expected story rule to produce a pair",
            ))
        })?;

        StoryAst::try_from(story_pair).map_err(Box::new)
    }
}
