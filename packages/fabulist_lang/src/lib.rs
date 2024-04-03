use pest::{error::LineColLocation, iterators::Pairs, Parser, RuleType};

#[derive(pest_derive::Parser)]
#[grammar = "./fabulist.pest"]
struct GrammarParser;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("[line {0}:{1}] {2}")]
    ParsingError(usize, usize, String),
}

pub struct FabulistLang;

impl FabulistLang {
    /// parses source string into pest's token pairs.
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

#[cfg(test)]
mod parser_tests {
    use pest::Parser;

    use crate::{GrammarParser, Rule};

    #[test]
    fn parses_primitives() {
        let result = GrammarParser::parse(Rule::number, "5");
        assert!(result.is_ok());
        let result = GrammarParser::parse(Rule::string, "\"sample string\"");
        assert!(result.is_ok());
        let result = GrammarParser::parse(Rule::boolean, "true");
        assert!(result.is_ok());
        let result = GrammarParser::parse(Rule::boolean, "false");
        assert!(result.is_ok());
        let result = GrammarParser::parse(Rule::identifier, "sample_identifier");
        assert!(result.is_ok());
    }

    #[test]
    fn parses_primaries() {
        let result = GrammarParser::parse(Rule::primary, "5");
        assert_eq!(
            result.clone().unwrap().next().unwrap().as_rule(),
            Rule::number
        );
        let result = GrammarParser::parse(Rule::primary, "\"sample string\"");
        assert_eq!(
            result.clone().unwrap().next().unwrap().as_rule(),
            Rule::string
        );
        let result = GrammarParser::parse(Rule::primary, "true");
        assert_eq!(
            result.clone().unwrap().next().unwrap().as_rule(),
            Rule::boolean
        );
        let result = GrammarParser::parse(Rule::primary, "sample_identifier");
        assert_eq!(
            result.clone().unwrap().next().unwrap().as_rule(),
            Rule::identifier
        );
        let result = GrammarParser::parse(Rule::primary, "(5 + 5)");
        assert_eq!(
            result.clone().unwrap().next().unwrap().as_rule(),
            Rule::grouping
        );
        let result = GrammarParser::parse(Rule::primary, "none");
        assert_eq!(
            result.clone().unwrap().next().unwrap().as_rule(),
            Rule::none
        );
        let result = GrammarParser::parse(Rule::primary, "{\"key\":\"value\"}");
        assert_eq!(
            result.clone().unwrap().next().unwrap().as_rule(),
            Rule::object
        );
        let result = GrammarParser::parse(Rule::primary, "(arg1, arg2)=>{ 5; }");
        assert_eq!(
            result.clone().unwrap().next().unwrap().as_rule(),
            Rule::lambda_function
        );
    }
}
