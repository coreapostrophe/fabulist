use pest::{iterators::Pairs, Parser};

use crate::error::Error;

#[derive(pest_derive::Parser)]
#[grammar = "../grammar/fabulist.pest"]
pub struct GrammarParser;

pub struct FabulistParser;

impl FabulistParser {
    pub fn parse(source: &str) -> Result<Pairs<'_, Rule>, Error> {
        GrammarParser::parse(Rule::fabulist, source).map_err(|error| Error::map_parser_error(error))
    }
}
