use pest::iterators::Pair;

use crate::parser::Rule;

use super::decl::{meta::MetaDecl, module::ModDecl, part::PartDecl};

use super::Error;

#[derive(Debug)]
pub struct Fab {
    pub module: Vec<ModDecl>,
    pub meta: Option<MetaDecl>,
    pub parts: Vec<PartDecl>,
}

impl TryFrom<Pair<'_, Rule>> for Fab {
    type Error = Error;
    fn try_from(value: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let mut module: Vec<ModDecl> = Vec::new();
        let mut meta: Option<MetaDecl> = None;
        let mut parts: Vec<PartDecl> = Vec::new();

        for pair in value.into_inner() {
            match pair.as_rule() {
                Rule::mod_decl => module.push(ModDecl::try_from(pair)?),
                Rule::part_decl => parts.push(PartDecl::try_from(pair)?),
                Rule::meta_decl => meta = Some(MetaDecl::try_from(pair)?),
                _ => (),
            }
        }

        Ok(Fab {
            module,
            meta,
            parts,
        })
    }
}

#[cfg(test)]
mod story_tests {
    use crate::ast::ParserTestHelper;

    use super::*;

    #[test]
    fn parses_story() {
        let test_helper = ParserTestHelper::<Fab>::new(Rule::fabulist, "Story");
        test_helper.assert_parse(
            r#"
			story {}

			# part_1
			[Jose]
			> "When are you getting a car?" => {
				"next": |>{
					set hello = "world";
				}
			}
			[Dave]
			> "Right, I was wondering about that as well."
				- "I'm flat out broke man." 
				- "In a few years. I just need to sell more coccaine. "
			"#,
        );
    }
}
