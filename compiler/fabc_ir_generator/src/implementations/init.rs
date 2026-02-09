use fabc_parser::ast::init::{module::ModuleInit, story::StoryInit, Init};

use crate::{GenerateIR, IRGenerator, IRResult};

impl GenerateIR for Init {
    fn generate_ir(&self, _generator: &mut IRGenerator) -> IRResult {
        todo!()
    }
}

impl GenerateIR for StoryInit {
    fn generate_ir(&self, _generator: &mut IRGenerator) -> IRResult {
        todo!()
    }
}

impl GenerateIR for ModuleInit {
    fn generate_ir(&self, _generator: &mut IRGenerator) -> IRResult {
        todo!()
    }
}
