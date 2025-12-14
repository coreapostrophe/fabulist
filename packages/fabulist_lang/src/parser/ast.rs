//! Abstract syntax tree modules for the Fabulist language.
#[cfg(test)]
use std::{fmt::Debug, marker::PhantomData};

#[cfg(test)]
use pest::iterators::Pair;

#[cfg(test)]
use crate::{
    interpreter::environment::{Environment, RuntimeEnvironment},
    interpreter::Evaluable,
    parser::Rule,
};

pub mod decl;
pub mod dfn;
pub mod expr;
pub mod stmt;
pub mod story;

#[cfg(test)]
/// Helper for parsing AST nodes inside unit tests.
pub(crate) struct AstTestHelper<T> {
    rule_type: Rule,
    struct_name: String,
    phantom: PhantomData<T>,
}

#[cfg(test)]
impl<'a, T> AstTestHelper<T>
where
    T: TryFrom<Pair<'a, Rule>> + Debug,
{
    /// Creates a helper bound to a pest rule and a descriptive name.
    pub fn new(rule_type: Rule, struct_name: impl Into<String>) -> Self {
        Self {
            rule_type,
            struct_name: struct_name.into(),
            phantom: PhantomData,
        }
    }

    /// Parses the source and returns the AST, panicking with a readable message on failure.
    pub fn assert_parse(&self, source: &'a str) -> T
    where
        T: TryFrom<Pair<'a, Rule>, Error = pest::error::Error<Rule>> + Debug + Clone,
    {
        use pest::Parser;

        use crate::parser::GrammarParser;

        match GrammarParser::parse(self.rule_type, source) {
            Err(error) => {
                println!("{}", error);
                panic!("Failed to parse source string");
            }
            Ok(mut parsing_result) => {
                let pair = parsing_result.next().unwrap_or_else(|| {
                    panic!("Failed to parse {} pair from string", self.struct_name)
                });
                match T::try_from(pair) {
                    Err(error) => {
                        println!("{}", error);
                        panic!("Failed to turn pair to `{}` struct", self.struct_name);
                    }
                    Ok(ast) => ast,
                }
            }
        }
    }
}

#[cfg(test)]
/// Options for evaluating parsed AST nodes inside tests.
pub(crate) struct AssertEvaluateOptions<'a> {
    /// Source string to parse.
    pub source: &'a str,
    /// Optional pre-seeded environment to use during evaluation.
    pub environment: Option<RuntimeEnvironment>,
    /// Optional pre-seeded story context.
    pub context: Option<RuntimeEnvironment>,
}

#[cfg(test)]
impl<'a, T> AstTestHelper<T>
where
    T: TryFrom<Pair<'a, Rule>> + Debug + Evaluable,
{
    /// Parses the source then evaluates the resulting AST within optional runtime scopes.
    pub fn parse_and_evaluate(&self, options: AssertEvaluateOptions<'a>) -> <T as Evaluable>::Output
    where
        T: TryFrom<Pair<'a, Rule>, Error = pest::error::Error<Rule>> + Debug + Clone,
    {
        let ast = self.assert_parse(options.source);
        let environment = options.environment.unwrap_or_else(Environment::new);
        let context = options.context.unwrap_or_else(Environment::new);

        ast.evaluate(&environment, &context)
    }
}
