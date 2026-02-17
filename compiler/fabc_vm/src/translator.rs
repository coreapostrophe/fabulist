use crate::instructions::Instruction;

pub mod implementations;

pub trait Translatable {
    fn translate(&self, translator: &mut AstTranslator) -> Vec<Instruction>;
}

pub struct AstTranslator;

impl AstTranslator {
    pub fn translate<T>(&mut self, node: &T) -> Vec<Instruction>
    where
        T: Translatable,
    {
        node.translate(self)
    }
}
