//! AST nodes for reusable definition fragments (objects, parameters, arguments).
use crate::{
    error::SpanSlice,
    parser::ast::expr::models::{Expr, IdentifierPrimitive},
};
use fabulist_derive::SyntaxTree;
use std::collections::HashMap;

/// Definition fragments reused across expressions and declarations.
#[derive(SyntaxTree, Debug, Clone)]
pub enum Dfn {
    /// Comma-delimited argument list following a call.
    #[production(span: SpanSlice, arguments: Option<Vec<Expr>>)]
    ArgumentBody(ArgumentBodyDfn),

    /// Comma-delimited parameter list for lambda-like constructs.
    #[production(span: SpanSlice, parameters: Option<Vec<IdentifierPrimitive>>)]
    ParameterBody(ParameterBodyDfn),

    /// Object literal expressed as a key/value mapping.
    #[production(span: SpanSlice, object: HashMap<String, Expr>)]
    Object(ObjectDfn),
}
