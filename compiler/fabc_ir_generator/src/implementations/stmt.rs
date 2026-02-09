use fabc_parser::ast::stmt::{
    block::BlockStmt,
    expr::ExprStmt,
    goto::GotoStmt,
    r#if::{ElseClause, IfStmt},
    r#let::LetStmt,
    r#return::ReturnStmt,
    Stmt,
};

use crate::{GenerateIR, IRGenerator, IRResult};

impl GenerateIR for Stmt {
    fn generate_ir(&self, _generator: &mut IRGenerator) -> IRResult {
        todo!()
    }
}

impl GenerateIR for BlockStmt {
    fn generate_ir(&self, _generator: &mut IRGenerator) -> IRResult {
        todo!()
    }
}

impl GenerateIR for ExprStmt {
    fn generate_ir(&self, _generator: &mut IRGenerator) -> IRResult {
        todo!()
    }
}

impl GenerateIR for LetStmt {
    fn generate_ir(&self, _generator: &mut IRGenerator) -> IRResult {
        todo!()
    }
}

impl GenerateIR for GotoStmt {
    fn generate_ir(&self, _generator: &mut IRGenerator) -> IRResult {
        todo!()
    }
}

impl GenerateIR for IfStmt {
    fn generate_ir(&self, _generator: &mut IRGenerator) -> IRResult {
        todo!()
    }
}

impl GenerateIR for ElseClause {
    fn generate_ir(&self, _generator: &mut IRGenerator) -> IRResult {
        todo!()
    }
}

impl GenerateIR for ReturnStmt {
    fn generate_ir(&self, _generator: &mut IRGenerator) -> IRResult {
        todo!()
    }
}
