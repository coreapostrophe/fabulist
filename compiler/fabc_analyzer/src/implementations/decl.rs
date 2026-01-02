use fabc_parser::ast::decl::{object::ObjectDecl, quote::QuoteDecl};

use crate::{Analyzable, Analyzer};

impl Analyzable for ObjectDecl {
    fn analyze(&self, _analyzer: &mut Analyzer) {}
}

impl Analyzable for QuoteDecl {
    fn analyze(&self, _analyzer: &mut Analyzer) {}
}
