use pest::{error::LineColLocation, iterators::Pairs, Parser, RuleType};

#[derive(pest_derive::Parser)]
#[grammar = "./fabulist.pest"]
pub struct GrammarParser;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("[line {0}:{1}] {2}")]
    ParsingError(usize, usize, String),
}

pub struct FabulistParser;

impl FabulistParser {
    pub fn parse(source: &str) -> Result<Pairs<'_, Rule>, Error> {
        GrammarParser::parse(Rule::fabulist, source).map_err(|error| Self::map_parser_error(error))
    }

    fn map_parser_error<R>(error: pest::error::Error<R>) -> Error
    where
        R: RuleType,
    {
        let message = error.variant.message();
        let (line, col) = match error.line_col {
            LineColLocation::Pos(line_col) => line_col,
            _ => (0, 0),
        };
        Error::ParsingError(line, col, message.into())
    }
}
