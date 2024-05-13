use pest::{iterators::Pairs, Parser};

#[derive(pest_derive::Parser)]
#[grammar = "../grammar/fabulist.pest"]
pub struct GrammarParser;

pub struct FabulistParser;

impl FabulistParser {
    pub fn parse(source: &str) -> Result<Pairs<'_, Rule>, pest::error::Error<Rule>> {
        GrammarParser::parse(Rule::story, source)
    }
}
