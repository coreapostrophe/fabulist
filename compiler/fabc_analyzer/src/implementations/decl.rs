use fabc_parser::ast::decl::{object::ObjectDecl, quote::QuoteDecl};

use crate::{error::Error, Analyzable, Analyzer};

impl Analyzable for ObjectDecl {
    fn analyze(&self, _analyzer: &mut Analyzer) -> Result<(), Error> {
        Ok(())
    }
}

impl Analyzable for QuoteDecl {
    fn analyze(&self, _analyzer: &mut Analyzer) -> Result<(), Error> {
        Ok(())
    }
}
