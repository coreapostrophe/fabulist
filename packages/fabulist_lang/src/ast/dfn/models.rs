use crate::{
    ast::expr::models::{Expr, IdentifierPrimitive},
    error::OwnedSpan,
};
use fabulist_derive::SyntaxTree;
use std::collections::HashMap;

#[derive(SyntaxTree, Debug, Clone)]
pub enum Dfn {
    #[production(span: OwnedSpan, arguments: Option<Vec<Expr>>)]
    ArgumentBody(ArgumentBodyDfn),

    #[production(span: OwnedSpan, parameters: Option<Vec<IdentifierPrimitive>>)]
    ParameterBody(ParameterBodyDfn),

    #[production(span: OwnedSpan, object: HashMap<String, Expr>)]
    Object(ObjectDfn),
}

#[cfg(test)]
mod dfn_tests {
    use crate::{ast::AstTestHelper, parser::Rule};

    use super::*;

    #[test]
    pub fn parses_parameter_body() {
        let test_helper =
            AstTestHelper::<ParameterBodyDfn>::new(Rule::parameter_body, "ParameterBodyDfn");
        test_helper.assert_parse(r#"(param1, param2, param3)"#);
    }

    #[test]
    pub fn parses_argument_body() {
        let test_helper =
            AstTestHelper::<ArgumentBodyDfn>::new(Rule::argument_body, "ArgumentBodyDfn");
        test_helper.assert_parse(r#"("string", 5, true)"#);
    }

    #[test]
    fn parses_object() {
        let test_helper = AstTestHelper::<ObjectDfn>::new(Rule::object, "ObjectDfn");
        test_helper.assert_parse(r#"{"boolean": false, "number": 10}"#);
    }
}
