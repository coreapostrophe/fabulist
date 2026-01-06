#![allow(unused)]

use fabc_parser::ast::decl::{object::ObjectDecl, quote::QuoteDecl};

use crate::Analyzable;

impl Analyzable for QuoteDecl {
    fn analyze(&self, _analyzer: &mut crate::Analyzer) {
        todo!()
    }
}

impl Analyzable for ObjectDecl {
    fn analyze(&self, _analyzer: &mut crate::Analyzer) {
        todo!()
    }
}
