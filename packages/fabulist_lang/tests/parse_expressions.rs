use fabulist_lang::parser::{GrammarParser, Rule};
use pest::Parser;

pub fn assert_primitive(rule: Rule, source: &str) {
    let result = GrammarParser::parse(rule, source);
    assert!(result.is_ok());
}

pub fn assert_primary(rule: Rule, source: &str) {
    let result = GrammarParser::parse(Rule::primary, source);
    let mut result = result.expect("Failed to parse source.");
    let primary = result.next().expect("Failed to extract primary pair.");
    let primary_rule = primary.as_rule();
    assert_eq!(primary_rule, rule);
}

pub fn assert_lambda_function(
    source: &str,
    parameter_assertions: Vec<&str>,
    statement_assertions: Vec<&str>,
) {
    let result = GrammarParser::parse(Rule::lambda_function, source);
    let mut result = result.expect("Failed to parse source.");
    let lambda_function = result
        .next()
        .expect("Failed to extract lambda_function pair.");
    let parameter_body = lambda_function
        .clone()
        .into_inner()
        .find(|p| p.as_rule() == Rule::parameter_body)
        .expect("Failed to extract parameter_body pair.");
    let parameters = parameter_body
        .clone()
        .into_inner()
        .find(|p| p.as_rule() == Rule::parameters);
    match parameters {
        Some(parameters) => {
            let parameter_array: Vec<_> = parameters.into_inner().map(|p| p.as_str()).collect();
            assert_eq!(parameter_array, parameter_assertions);
        }
        _ => (),
    }
    let statement_body = lambda_function
        .clone()
        .into_inner()
        .find(|p| p.as_rule() == Rule::statement_body)
        .expect("Failed to extract statement_body pair.");
    let statements = statement_body
        .clone()
        .into_inner()
        .find(|p| p.as_rule() == Rule::control_statement);
    match statements {
        Some(statements) => {
            let statement_array: Vec<_> = statements.into_inner().map(|p| p.as_str()).collect();
            assert_eq!(statement_array, statement_assertions);
        }
        _ => (),
    }
}

pub fn assert_call(source: &str, callee_assertion: &str, argument_assertions: Vec<&str>) {
    let result = GrammarParser::parse(Rule::call, source);
    let mut result = result.expect("Failed to parse source.");
    let call = result.next().expect("Failed to extract call pair.");
    let callee = call
        .clone()
        .into_inner()
        .find_tagged("callee")
        .next()
        .expect("Failed to extract callee pair.");
    assert_eq!(callee.as_str(), callee_assertion);
    let argument_body = call
        .into_inner()
        .find(|p| p.as_rule() == Rule::argument_body)
        .expect("Failed to extract argument_body pair.");
    let arguments = argument_body
        .clone()
        .into_inner()
        .find(|p| p.as_rule() == Rule::arguments);
    match arguments {
        Some(arguments) => {
            let argument_array: Vec<_> = arguments.into_inner().map(|p| p.as_str()).collect();
            assert_eq!(argument_array, argument_assertions);
        }
        _ => (),
    }
}

pub fn assert_unary(source: &str, operator_assertion: &str, right_assertion: &str) {
    let result = GrammarParser::parse(Rule::unary, source);
    let mut result = result.expect("Failed to parse source.");
    let unary = result.next().expect("Failed to extract unary pair.");
    let operator = unary
        .clone()
        .into_inner()
        .find_tagged("operator")
        .next()
        .expect("Failed to extract operator pair.");
    assert_eq!(operator.as_str(), operator_assertion);
    let right = unary
        .clone()
        .into_inner()
        .find_tagged("right")
        .next()
        .expect("Failed to extract right pair.");
    assert_eq!(right.as_str(), right_assertion);
}

pub fn assert_binomial_operation(rule: Rule, source: &str, assertions: [&str; 3]) {
    let [left_assertion, operator_assertion, right_assertion] = assertions;
    let result = GrammarParser::parse(rule, source);
    let mut result = result.expect("Failed to parse source.");
    let binomial_operation = result
        .next()
        .expect("Failed to extract binomial_operation pair.");
    let left = binomial_operation
        .clone()
        .into_inner()
        .find_tagged("left")
        .next()
        .expect("Failed to extract left pair.");
    assert_eq!(left.as_str(), left_assertion);
    let operator = binomial_operation
        .clone()
        .into_inner()
        .find_tagged("operator")
        .next()
        .expect("Failed to extract operator pair.");
    assert_eq!(operator.as_str(), operator_assertion);
    let right = binomial_operation
        .clone()
        .into_inner()
        .find_tagged("right")
        .next()
        .expect("Failed to extract right pair.");
    assert_eq!(right.as_str(), right_assertion);
}

pub fn assert_assignment(source: &str, name_assertion: &str, value_assertion: &str) {
    let result = GrammarParser::parse(Rule::assignment, source);
    let mut result = result.expect("Failed to parse source.");
    let assignment = result.next().expect("Failed to extract assignment pair.");
    let name = assignment
        .clone()
        .into_inner()
        .find_tagged("name")
        .next()
        .expect("Failed to extract name pair.");
    assert_eq!(name.as_str(), name_assertion);
    let value = assignment
        .clone()
        .into_inner()
        .find_tagged("value")
        .next()
        .expect("Failed to extract value pair.");
    assert_eq!(value.as_str(), value_assertion);
}

#[test]
pub fn parses_primitives() {
    assert_primitive(Rule::number, "5");
    assert_primitive(Rule::string, "\"sample string\"");
    assert_primitive(Rule::boolean, "true");
    assert_primitive(Rule::boolean, "false");
    assert_primitive(Rule::identifier, "sample_identifier");
    assert_primitive(Rule::raw_string, "r#\"sample raw string\"#");
}

#[test]
pub fn parses_primaries() {
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
pub fn parses_lambda_functions() {
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
pub fn parses_calls() {
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
pub fn parses_unary() {
    assert_unary("!5", "!", "5");
    assert_unary("-5", "-", "5");
    assert_unary("-(5 + 5)", "-", "(5 + 5)");
    assert_unary("-(  2 > false )", "-", "(  2 > false )");
}

#[test]
pub fn parses_factor() {
    assert_binomial_operation(Rule::factor, "5 /5", ["5 ", "/", "5"]);
    assert_binomial_operation(Rule::factor, "5 *(1+2)", ["5 ", "*", "(1+2)"]);
    assert_binomial_operation(Rule::factor, "true /5", ["true ", "/", "5"]);
    assert_binomial_operation(Rule::factor, "8* false", ["8", "*", "false"]);
}

#[test]
pub fn parses_term() {
    assert_binomial_operation(Rule::term, "1 +7", ["1 ", "+", "7"]);
    assert_binomial_operation(Rule::term, "6 -   9", ["6 ", "-", "9"]);
    assert_binomial_operation(Rule::term, "3 + (5)", ["3 ", "+", "(5)"]);
    assert_binomial_operation(Rule::term, "5-3", ["5", "-", "3"]);
}

#[test]
pub fn parses_comparison() {
    assert_binomial_operation(Rule::comparison, "6 < 7", ["6 ", "<", "7"]);
    assert_binomial_operation(Rule::comparison, "8<= (7)(true)", ["8", "<=", "(7)(true)"]);
    assert_binomial_operation(Rule::comparison, "1    >2", ["1    ", ">", "2"]);
    assert_binomial_operation(Rule::comparison, "0>= (1 + 5)", ["0", ">=", "(1 + 5)"]);
}

#[test]
pub fn parses_equality() {
    assert_binomial_operation(Rule::equality, "2 == 9", ["2 ", "==", "9"]);
    assert_binomial_operation(Rule::equality, "6    != false", ["6    ", "!=", "false"]);
    assert_binomial_operation(Rule::equality, "4==(i)=>{i;}", ["4", "==", "(i)=>{i;}"]);
    assert_binomial_operation(Rule::equality, "7 != true", ["7 ", "!=", "true"]);
}

#[test]
pub fn parses_logical() {
    assert_binomial_operation(Rule::logical, "1&&1", ["1", "&&", "1"]);
    assert_binomial_operation(Rule::logical, "yo ||   yo1", ["yo ", "||", "yo1"]);
    assert_binomial_operation(Rule::logical, "true && false", ["true ", "&&", "false"]);
    assert_binomial_operation(Rule::logical, "hello|| test()", ["hello", "||", "test()"]);
}

#[test]
pub fn parses_assignment() {
    assert_assignment("num = 5", "num", "5");
    assert_assignment("callback = (arg) => {arg;}", "callback", "(arg) => {arg;}");
    assert_assignment("flag = false", "flag", "false");
    assert_assignment("text = \"Hello world!\"", "text", "\"Hello world!\"");
}
