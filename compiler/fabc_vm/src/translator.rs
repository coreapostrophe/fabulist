use crate::instructions::Instruction;

pub mod implementations;

pub trait Translatable {
    fn translate_with(&self, translator: &mut AstTranslator, buffer: &mut Vec<Instruction>);

    fn translate(&self, translator: &mut AstTranslator) -> Vec<Instruction> {
        let mut buffer = Vec::new();
        self.translate_with(translator, &mut buffer);
        buffer
    }
}

pub struct AstTranslator;

impl AstTranslator {
    pub fn translate<T>(node: &T) -> Vec<Instruction>
    where
        T: Translatable,
    {
        let translator = &mut AstTranslator;
        node.translate(translator)
    }
}
