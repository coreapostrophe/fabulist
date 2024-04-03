use pest::{error::LineColLocation, iterators::Pairs, Parser, RuleType};

#[derive(pest_derive::Parser)]
#[grammar = "./fabulist.pest"]
struct GrammarParser;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("[line {0}:{1}] {2}")]
    ParsingError(usize, usize, String),
}

pub struct FabulistParser;

impl FabulistParser {
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

    use crate::parser::{GrammarParser, Rule};

    macro_rules! assert_primitive {
        ($rule:expr, $source:expr) => {
            let result = GrammarParser::parse($rule, $source);
            assert!(result.is_ok());
        };
    }

    macro_rules! assert_primary {
        ($rule:expr, $source:expr) => {
            let result = GrammarParser::parse(Rule::primary, $source);
            assert_eq!(result.clone().unwrap().next().unwrap().as_rule(), $rule);
        };
    }

    macro_rules! assert_call {
        ($source:expr, $callee:expr, $($arg:expr),*) => {
            let result = GrammarParser::parse(Rule::call, $source);
            assert_eq!(
                result.clone().unwrap().next().unwrap().as_rule(),
                Rule::call
            );
            assert_eq!(
                result
                    .clone()
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
                    .find(|p| p.as_node_tag().unwrap() == "callee")
                    .unwrap()
                    .as_str(),
                $callee
            );
            assert_eq!(
                result
                    .clone()
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
                    .find(|p| p.as_rule() == Rule::arguments)
                    .unwrap()
                    .into_inner()
                    .map(|p| p.as_str())
                    .collect::<Vec<_>>(),
                $($arg)*
            );
        };
    }

    macro_rules! assert_unary {
        ($source:expr, $operator:expr, $right:expr) => {
            let result = GrammarParser::parse(Rule::unary, $source);
            assert_eq!(
                result
                    .clone()
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
                    .find_tagged("operator")
                    .next()
                    .unwrap()
                    .as_str(),
                $operator
            );
            assert_eq!(
                result
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
                    .find_tagged("right")
                    .next()
                    .unwrap()
                    .as_str(),
                $right
            );
        };
    }

    macro_rules! assert_factor {
        ($source:expr, $left:expr, $operator:expr, $right:expr) => {
            let result = GrammarParser::parse(Rule::factor, $source);
            assert_eq!(
                result
                    .clone()
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
                    .find_tagged("left")
                    .next()
                    .unwrap()
                    .as_str(),
                $left
            );
            assert_eq!(
                result
                    .clone()
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
                    .find_tagged("operator")
                    .next()
                    .unwrap()
                    .as_str(),
                $operator
            );
            assert_eq!(
                result
                    .clone()
                    .unwrap()
                    .next()
                    .unwrap()
                    .into_inner()
                    .find_tagged("right")
                    .next()
                    .unwrap()
                    .as_str(),
                $right
            );
        };
    }

    #[test]
    fn parses_primitives() {
        assert_primitive!(Rule::number, "5");
        assert_primitive!(Rule::string, "\"sample string\"");
        assert_primitive!(Rule::boolean, "true");
        assert_primitive!(Rule::boolean, "false");
        assert_primitive!(Rule::identifier, "sample_identifier");
    }

    #[test]
    fn parses_primaries() {
        assert_primary!(Rule::number, "5");
        assert_primary!(Rule::string, "\"sample string\"");
        assert_primary!(Rule::boolean, "true");
        assert_primary!(Rule::boolean, "false");
        assert_primary!(Rule::identifier, "sample_identifier");
        assert_primary!(Rule::grouping, "(5 + 5)");
        assert_primary!(Rule::none, "none");
        assert_primary!(Rule::object, "{\"key\":\"value\"}");
        assert_primary!(Rule::lambda_function, "(arg1, arg2)=>{ 5; }");
    }

    #[test]
    fn parses_calls() {
        assert_call!("sum(addend1, addend2)", "sum", ["addend1", "addend2"]);
        assert_call!(
            "subtraction(minuend, subtrahend)",
            "subtraction",
            ["minuend", "subtrahend"]
        );
        assert_call!(
            "division(dividend, divisor)",
            "division",
            ["dividend", "divisor"]
        );
        assert_call!(
            "multiplication(multiplicand, multiplier)",
            "multiplication",
            ["multiplicand", "multiplier"]
        );
    }

    #[test]
    fn parses_unary() {
        assert_unary!("!5", "!", "5");
        assert_unary!("-5", "-", "5");
        assert_unary!("-(5 + 5)", "-", "(5 + 5)");
        assert_unary!("-(  2 > false )", "-", "(  2 > false )");
    }

    #[test]
    fn parses_factor() {
        assert_factor!("5 /5", "5 ", "/", "5");
        assert_factor!("5 *(1+2)", "5 ", "*", "(1+2)");
        assert_factor!("true /5", "true ", "/", "5");
    }
}
