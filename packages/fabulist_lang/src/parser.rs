use pest::{error::LineColLocation, iterators::Pairs, Parser, RuleType};

mod test_helpers;

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
    use super::{test_helpers::*, Rule};

    #[test]
    fn parses_primitives() {
        assert_primitive(Rule::number, "5");
        assert_primitive(Rule::string, "\"sample string\"");
        assert_primitive(Rule::boolean, "true");
        assert_primitive(Rule::boolean, "false");
        assert_primitive(Rule::identifier, "sample_identifier");
        assert_primitive(Rule::raw_string, "r#\"sample raw string\"#");
    }

    #[test]
    fn parses_primaries() {
        assert_primary(Rule::number, "5");
        assert_primary(Rule::string, "\"sample string\"");
        assert_primary(Rule::boolean, "true");
        assert_primary(Rule::boolean, "false");
        assert_primary(Rule::identifier, "sample_identifier");
        assert_primary(Rule::grouping, "(5 + 5)");
        assert_primary(Rule::none, "none");
        assert_primary(Rule::object, "{\"key\":\"value\"}");
        assert_primary(Rule::lambda_function, "(arg1, arg2)=>{ 5; }");
        assert_primary(Rule::raw_string, "r##\"sample raw string\"##");
    }

    #[test]
    fn parses_lambda_functions() {
        assert_lambda_function("() => {}", vec![], vec![]);
        assert_lambda_function(
            "(param1, param2) => {     }",
            vec!["param1", "param2"],
            vec![],
        );
        assert_lambda_function(
            "() =>{     statement1; statement2;}",
            vec![],
            vec!["statement1", "statement2"],
        );
        assert_lambda_function(
            "(param1,param2)=>{param1;param2;}",
            vec!["param1", "param2"],
            vec!["param1;", "param2;"],
        );
    }

    #[test]
    fn parses_calls() {
        assert_call("sum(addend1, addend2)", "sum", vec!["addend1", "addend2"]);
        assert_call(
            "subtraction(minuend, subtrahend)",
            "subtraction",
            vec!["minuend", "subtrahend"],
        );
        assert_call(
            "division(dividend, divisor)",
            "division",
            vec!["dividend", "divisor"],
        );
        assert_call(
            "multiplication(multiplicand, multiplier)",
            "multiplication",
            vec!["multiplicand", "multiplier"],
        );
    }

    #[test]
    fn parses_unary() {
        assert_unary("!5", "!", "5");
        assert_unary("-5", "-", "5");
        assert_unary("-(5 + 5)", "-", "(5 + 5)");
        assert_unary("-(  2 > false )", "-", "(  2 > false )");
    }

    #[test]
    fn parses_factor() {
        assert_binomial_operation(Rule::factor, "5 /5", ["5 ", "/", "5"]);
        assert_binomial_operation(Rule::factor, "5 *(1+2)", ["5 ", "*", "(1+2)"]);
        assert_binomial_operation(Rule::factor, "true /5", ["true ", "/", "5"]);
    }

    #[test]
    fn parses_term() {
        assert_binomial_operation(Rule::term, "1 +7", ["1 ", "+", "7"]);
        assert_binomial_operation(Rule::term, "6 -   9", ["6 ", "-", "9"]);
        assert_binomial_operation(Rule::term, "3 + (5)", ["3 ", "+", "(5)"]);
        assert_binomial_operation(Rule::term, "5-3", ["5", "-", "3"]);
    }

    #[test]
    fn parses_comparison() {
        assert_binomial_operation(Rule::comparison, "6 < 7", ["6 ", "<", "7"]);
        assert_binomial_operation(Rule::comparison, "8<= (7)(true)", ["8", "<=", "(7)(true)"]);
        assert_binomial_operation(Rule::comparison, "1    >2", ["1    ", ">", "2"]);
        assert_binomial_operation(Rule::comparison, "0>= (1 + 5)", ["0", ">=", "(1 + 5)"]);
    }

    #[test]
    fn parses_equality() {
        assert_binomial_operation(Rule::equality, "2 == 9", ["2 ", "==", "9"]);
        assert_binomial_operation(Rule::equality, "6    != false", ["6    ", "!=", "false"]);
        assert_binomial_operation(Rule::equality, "4==(i)=>{i;}", ["4", "==", "(i)=>{i;}"]);
        assert_binomial_operation(Rule::equality, "7 != true", ["7 ", "!=", "true"]);
    }

    #[test]
    fn parses_logical() {
        assert_binomial_operation(Rule::logical, "1&&1", ["1", "&&", "1"]);
        assert_binomial_operation(Rule::logical, "yo ||   yo1", ["yo ", "||", "yo1"]);
        assert_binomial_operation(Rule::logical, "true && false", ["true ", "&&", "false"]);
        assert_binomial_operation(Rule::logical, "hello|| test()", ["hello", "||", "test()"]);
    }

    #[test]
    fn parses_assignment() {
        assert_assignment("num = 5", "num", "5");
        assert_assignment("callback = (arg) => {arg;}", "callback", "(arg) => {arg;}");
        assert_assignment("flag = false", "flag", "false");
        assert_assignment("text = \"Hello world!\"", "text", "\"Hello world!\"");
    }
}
