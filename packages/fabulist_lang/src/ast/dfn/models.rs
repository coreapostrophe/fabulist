//! AST nodes for reusable definition fragments (objects, parameters, arguments).
use crate::{
    ast::expr::models::{Expr, IdentifierPrimitive},
    error::OwnedSpan,
};
use fabulist_derive::SyntaxTree;
use std::collections::HashMap;

/// Definition fragments reused across expressions and declarations.
#[derive(SyntaxTree, Debug, Clone)]
pub enum Dfn {
    /// Comma-delimited argument list following a call.
    #[production(span: OwnedSpan, arguments: Option<Vec<Expr>>)]
    ArgumentBody(ArgumentBodyDfn),

    /// Comma-delimited parameter list for lambda-like constructs.
    #[production(span: OwnedSpan, parameters: Option<Vec<IdentifierPrimitive>>)]
    ParameterBody(ParameterBodyDfn),

    /// Object literal expressed as a key/value mapping.
    #[production(span: OwnedSpan, object: HashMap<String, Expr>)]
    Object(ObjectDfn),
}
