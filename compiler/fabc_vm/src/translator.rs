use std::collections::HashMap;

use fabc_analyzer::{
    types::{BindingDetails, ModuleSymbolType, SymbolAnnotation},
    AnalyzerResult,
};

use crate::instructions::Instruction;

pub mod implementations;

pub trait Translatable {
    fn translate_with(&self, translator: &mut AstTranslator, buffer: &mut Vec<Instruction>);

    fn translate(&self, translator: &mut AstTranslator) -> Vec<Instruction> {
        let mut buffer = Vec::new();
        self.translate_with(translator, &mut buffer);
        translator.finalize(buffer)
    }
}

#[derive(Default)]
pub struct AstTranslator {
    mod_annotations: HashMap<usize, SymbolAnnotation<ModuleSymbolType>>,
    frame_size: usize,
}

impl AstTranslator {
    pub fn translate<T>(node: &T) -> Vec<Instruction>
    where
        T: Translatable,
    {
        let translator = &mut AstTranslator::default();
        node.translate(translator)
    }

    pub fn translate_with_annotations<T>(
        node: &T,
        mod_annotations: HashMap<usize, SymbolAnnotation<ModuleSymbolType>>,
    ) -> Vec<Instruction>
    where
        T: Translatable,
    {
        let translator = &mut AstTranslator::with_mod_annotations(mod_annotations);
        node.translate(translator)
    }

    pub fn from_analyzer(result: AnalyzerResult) -> Self {
        let frame_size = result
            .mod_sym_annotations
            .values()
            .filter_map(|annotation| annotation.binding.as_ref())
            .map(|binding| binding.slot + 1)
            .max()
            .unwrap_or(0);

        Self {
            mod_annotations: result.mod_sym_annotations,
            frame_size,
        }
    }

    pub fn with_mod_annotations(
        mod_annotations: HashMap<usize, SymbolAnnotation<ModuleSymbolType>>,
    ) -> Self {
        let frame_size = mod_annotations
            .values()
            .filter_map(|annotation| annotation.binding.as_ref())
            .map(|binding| binding.slot + 1)
            .max()
            .unwrap_or(0);

        Self {
            mod_annotations,
            frame_size,
        }
    }

    fn resolve_binding(&self, node_id: usize) -> Option<&BindingDetails> {
        self.mod_annotations
            .get(&node_id)
            .and_then(|annotation| annotation.binding.as_ref())
    }

    fn finalize(&self, mut buffer: Vec<Instruction>) -> Vec<Instruction> {
        let mut instructions = Vec::with_capacity(self.frame_size + buffer.len() + 2);

        instructions.push(Instruction::EnterFrame(self.frame_size));
        instructions.append(&mut buffer);
        instructions.push(Instruction::ExitFrame);

        instructions
    }
}
