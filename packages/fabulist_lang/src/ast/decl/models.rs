use crate::{
    ast::{dfn::models::ObjectDfn, expr::models::IdentifierPrimitive},
    error::OwnedSpan,
};
use fabulist_derive::SyntaxTree;

#[derive(SyntaxTree, Debug, Clone)]
pub enum Element {
    #[production(span: OwnedSpan, value: DialogueDecl)]
    Dialogue(DialogueElement),

    #[production(span: OwnedSpan, value: QuoteDecl)]
    Choice(ChoiceElement),

    #[production(span: OwnedSpan, value: QuoteDecl)]
    Narration(NarrationElement),
}

#[derive(SyntaxTree, Debug, Clone)]
pub enum Decl {
    #[production(span: OwnedSpan, text: String, properties: Option<ObjectDfn>)]
    Quote(QuoteDecl),

    #[production(span: OwnedSpan, character: String, quotes: Vec<QuoteDecl>)]
    Dialogue(DialogueDecl),

    #[production(span: OwnedSpan, value: Element)]
    Element(ElementDecl),

    #[production(span: OwnedSpan, properties: ObjectDfn)]
    Meta(MetaDecl),

    #[production(span: OwnedSpan, path: String, identifier: IdentifierPrimitive)]
    Module(ModuleDecl),

    #[production(span: OwnedSpan, id: String, elements: Vec<ElementDecl>)]
    Part(PartDecl),
}
