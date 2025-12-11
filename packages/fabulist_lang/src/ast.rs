#[cfg(test)]
use std::{fmt::Debug, marker::PhantomData};

#[cfg(test)]
use pest::{iterators::Pair, Parser};

#[cfg(test)]
use crate::parser::{GrammarParser, Rule};

pub mod decl;
pub mod dfn;
pub mod expr;
pub mod stmt;
pub mod story;

#[cfg(test)]
pub struct AstTestHelper<T> {
    rule_type: Rule,
    struct_name: String,
    phantom: PhantomData<T>,
}

#[cfg(test)]
impl<'a, T> AstTestHelper<T>
where
    T: TryFrom<Pair<'a, Rule>> + Debug,
{
    pub fn new(rule_type: Rule, struct_name: impl Into<String>) -> Self {
        Self {
            rule_type,
            struct_name: struct_name.into(),
            phantom: PhantomData,
        }
    }

    pub fn assert_parse(&self, source: &'a str) -> T
    where
        T: TryFrom<Pair<'a, Rule>, Error = pest::error::Error<Rule>> + Debug + Clone,
    {
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
