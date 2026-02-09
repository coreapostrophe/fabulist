use fabc_parser::ast::decl::{object::ObjectDecl, quote::QuoteDecl};

use crate::{GenerateIR, IRGenerator, IRResult};

impl GenerateIR for ObjectDecl {
    fn generate_ir(&self, _generator: &mut IRGenerator) -> IRResult {
        todo!()
    }
}

impl GenerateIR for QuoteDecl {
    fn generate_ir(&self, _generator: &mut IRGenerator) -> IRResult {
        todo!()
    }
}
