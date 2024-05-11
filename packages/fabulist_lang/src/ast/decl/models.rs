use crate::ast::{dfn::models::ObjectDfn, expr::models::IdentifierPrimitive};
use fabulist_derive::SyntaxTree;

#[derive(SyntaxTree, Debug, Clone)]
pub enum Element {
    #[production(value: DialogueDecl)]
    Dialogue(DialogueElement),

    #[production(value: QuoteDecl)]
    Choice(ChoiceElement),

    #[production(value: QuoteDecl)]
    Narration(NarrationElement),
}

#[derive(SyntaxTree, Debug, Clone)]
pub enum Decl {
    #[production(text: String, properties: Option<ObjectDfn>)]
    Quote(QuoteDecl),

    #[production(character: String, quotes: Vec<QuoteDecl>)]
    Dialogue(DialogueDecl),

    #[production(value: Element)]
    Element(ElementDecl),

    #[production(properties: ObjectDfn)]
    Meta(MetaDecl),

    #[production(path: String, identifier: IdentifierPrimitive)]
    Module(ModuleDecl),

    #[production(id: String, elements: Vec<ElementDecl>)]
    Part(PartDecl),
}

#[cfg(test)]
mod decl_tests {
    use crate::{ast::AstTestHelper, parser::Rule};

    use super::*;

    #[test]
    fn parses_quote_elem() {
        let test_helper = AstTestHelper::<QuoteDecl>::new(Rule::quote_decl, "QuoteDecl");
        test_helper.assert_parse(r#"> "I'm an example quote""#);

        let test_helper = AstTestHelper::<QuoteDecl>::new(Rule::narration_decl, "QuoteDecl");
        test_helper.assert_parse(r#"* "I'm an example narration""#);

        let test_helper = AstTestHelper::<QuoteDecl>::new(Rule::choice_decl, "QuoteDecl");
        test_helper.assert_parse(r#"- "I'm an example choice""#);
    }

    #[test]
    fn parses_dialogue_elem() {
        let test_helper = AstTestHelper::<DialogueDecl>::new(Rule::dialogue_decl, "DialogueDecl");
        test_helper.assert_parse(r#"[char] > "I'm a dialogue" > "I'm another dialogue""#);
    }

    #[test]
    fn parses_element_stmt() {
        let test_helper = AstTestHelper::<Element>::new(Rule::element_decl, "ElementDecl");
        test_helper.assert_parse(r#"[char]> "I'm a dialogue""#);
        test_helper.assert_parse(r#"* "I'm a narration""#);
        test_helper.assert_parse(r#"- "I'm a choice""#);
    }

    #[test]
    fn parses_meta_stmt() {
        let test_helper = AstTestHelper::<MetaDecl>::new(Rule::meta_decl, "MetaDecl");
        test_helper.assert_parse(r#"story { "start": "part-1" }"#);
    }

    #[test]
    fn parses_module_tests() {
        let test_helper = AstTestHelper::<ModuleDecl>::new(Rule::mod_decl, "ModDecl");
        test_helper.assert_parse("module \"./module.fab\" as module_1;");
    }

    #[test]
    fn parses_part_stmt() {
        let test_helper = AstTestHelper::<PartDecl>::new(Rule::part_decl, "PartDecl");
        test_helper.assert_parse(r#"#ident-1 [char]>"I'm a dialogue""#);
    }
}
