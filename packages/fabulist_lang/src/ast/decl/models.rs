//! AST nodes for declarations: quotes, dialogues, parts, modules, and metadata.
use crate::{
    ast::{dfn::models::ObjectDfn, expr::models::IdentifierPrimitive},
    error::OwnedSpan,
};
use fabulist_derive::SyntaxTree;

/// Narrative elements allowed inside a part.
#[derive(SyntaxTree, Debug, Clone)]
pub enum Element {
    /// Dialogue block initiated by a character declaration.
    #[production(span: OwnedSpan, value: DialogueDecl)]
    Dialogue(DialogueElement),

    /// Choice presented to the audience.
    #[production(span: OwnedSpan, value: QuoteDecl)]
    Choice(ChoiceElement),

    /// Narration line presented without character attribution.
    #[production(span: OwnedSpan, value: QuoteDecl)]
    Narration(NarrationElement),
}

/// Top-level declarations that can appear in a story file.
#[derive(SyntaxTree, Debug, Clone)]
pub enum Decl {
    /// A single line of dialogue, narration, or choice.
    #[production(span: OwnedSpan, text: String, properties: Option<ObjectDfn>)]
    Quote(QuoteDecl),

    /// Group of quotes attributed to a character.
    #[production(span: OwnedSpan, character: String, quotes: Vec<QuoteDecl>)]
    Dialogue(DialogueDecl),

    /// Wrapper for an embedded [`Element`].
    #[production(span: OwnedSpan, value: Element)]
    Element(ElementDecl),

    /// Story-level metadata block (`story { ... }`).
    #[production(span: OwnedSpan, properties: ObjectDfn)]
    Meta(MetaDecl),

    /// External module import (`module "./file.fab" as alias;`).
    #[production(span: OwnedSpan, path: String, identifier: IdentifierPrimitive)]
    Module(ModuleDecl),

    /// Story part introduced with a `#` heading.
    #[production(span: OwnedSpan, id: String, elements: Vec<ElementDecl>)]
    Part(PartDecl),
}
