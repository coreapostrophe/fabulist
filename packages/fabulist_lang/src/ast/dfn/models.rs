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
