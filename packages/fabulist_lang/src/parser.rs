//! Pest-based parser entry points for Fabulist source code.
#![allow(missing_docs)]

use pest::Parser;

use crate::parser::{
    ast::story::StoryAst,
    error::{ParserError, ParserResult},
};

pub mod ast;
pub mod error;

#[derive(pest_derive::Parser)]
#[grammar = "../grammar/fabulist.pest"]
/// Generated parser for the Fabulist grammar.
pub struct FabulistPestParser;

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
    pub fn parse(source: impl Into<String>) -> ParserResult<StoryAst> {
        let source = source.into();

        let mut pairs = FabulistPestParser::parse(Rule::story, &source).map_err(Box::new)?;
        let story_pair = pairs.next().ok_or(ParserError::UnableToParseStory)?;

        StoryAst::try_from(story_pair)
    }
}
