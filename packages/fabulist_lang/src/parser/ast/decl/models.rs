//! AST nodes for declarations: quotes, dialogues, parts, modules, and metadata.
use crate::parser::{
    ast::{dfn::models::ObjectDfn, expr::models::IdentifierPrimitive},
    error::SpanSlice,
};
use fabulist_derive::SyntaxTree;

/// Narrative elements allowed inside a part.
#[derive(SyntaxTree, Debug, Clone)]
pub enum Element {
    /// Dialogue block initiated by a character declaration.
    #[production(span_slice: SpanSlice, value: DialogueDecl)]
    Dialogue(DialogueElement),

    /// Choice presented to the audience.
    #[production(span_slice: SpanSlice, quote: QuoteDecl)]
    Choice(ChoiceElement),

    /// Narration line presented without character attribution.
    #[production(span_slice: SpanSlice, quote: QuoteDecl)]
    Narration(NarrationElement),
}

/// Top-level declarations that can appear in a story file.
#[derive(SyntaxTree, Debug, Clone)]
pub enum Decl {
    /// A single line of dialogue, narration, or choice.
    #[production(span_slice: SpanSlice, text: String, properties: Option<ObjectDfn>)]
    Quote(QuoteDecl),

    /// Group of quotes attributed to a character.
    #[production(span_slice: SpanSlice, character: String, quotes: Vec<QuoteDecl>)]
    Dialogue(DialogueDecl),

    /// Wrapper for an embedded [`Element`].
    #[production(span_slice: SpanSlice, value: Element)]
    Element(ElementDecl),

    /// Story-level metadata block (`story { ... }`).
    #[production(span_slice: SpanSlice, properties: ObjectDfn)]
    Meta(MetaDecl),

    /// External module import (`module "./file.fab" as alias;`).
    #[production(span_slice: SpanSlice, path: String, identifier: IdentifierPrimitive)]
    Module(ModuleDecl),

    /// Story part introduced with a `#` heading.
    #[production(span_slice: SpanSlice, id: String, elements: Vec<ElementDecl>)]
    Part(PartDecl),
}
