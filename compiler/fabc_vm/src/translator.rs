use std::collections::HashMap;

use fabc_analyzer::{
    types::{BindingDetails, BindingKind, ModuleSymbolType, SymbolAnnotation},
    AnalyzerResult,
};
use fabc_error::Error;

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
    errors: Vec<Error>,
}

pub struct TranslatorResult {
    pub instructions: Vec<Instruction>,
    pub errors: Vec<Error>,
}

impl AstTranslator {
    pub fn translate<T>(node: &T) -> Vec<Instruction>
    where
        T: Translatable,
    {
        Self::translate_result(node).instructions
    }

    pub fn translate_result<T>(node: &T) -> TranslatorResult
    where
        T: Translatable,
    {
        let mut translator = AstTranslator::default();
        let instructions = node.translate(&mut translator);

        TranslatorResult {
            instructions,
            errors: translator.errors,
        }
    }

    pub fn translate_with_annotations<T>(
        node: &T,
        mod_annotations: HashMap<usize, SymbolAnnotation<ModuleSymbolType>>,
    ) -> Vec<Instruction>
    where
        T: Translatable,
    {
        Self::translate_with_annotations_result(node, mod_annotations).instructions
    }

    pub fn translate_with_annotations_result<T>(
        node: &T,
        mod_annotations: HashMap<usize, SymbolAnnotation<ModuleSymbolType>>,
    ) -> TranslatorResult
    where
        T: Translatable,
    {
        let mut translator = AstTranslator::with_mod_annotations(mod_annotations);
        let instructions = node.translate(&mut translator);

        TranslatorResult {
            instructions,
            errors: translator.errors,
        }
    }

    pub fn from_analyzer(result: AnalyzerResult) -> Self {
        let (frame_size, _) = Self::compute_sizes(result.mod_sym_annotations.values());

        Self {
            mod_annotations: result.mod_sym_annotations,
            frame_size,
            errors: Vec::new(),
        }
    }

    pub fn with_mod_annotations(
        mod_annotations: HashMap<usize, SymbolAnnotation<ModuleSymbolType>>,
    ) -> Self {
        let (frame_size, _) = Self::compute_sizes(mod_annotations.values());

        Self {
            mod_annotations,
            frame_size,
            errors: Vec::new(),
        }
    }

    fn resolve_binding(&self, node_id: usize) -> Option<&BindingDetails> {
        self.mod_annotations
            .get(&node_id)
            .and_then(|annotation| annotation.binding.as_ref())
    }

    pub fn errors(&self) -> &[Error] {
        &self.errors
    }

    fn push_error(&mut self, error: Error) {
        self.errors.push(error);
    }

    fn finalize(&self, mut buffer: Vec<Instruction>) -> Vec<Instruction> {
        let mut instructions = Vec::with_capacity(buffer.len() + 2);

        instructions.push(Instruction::EnterFrame(self.frame_size));
        instructions.append(&mut buffer);
        instructions.push(Instruction::ExitFrame);

        instructions
    }

    fn compute_sizes<'a>(
        annotations: impl Iterator<Item = &'a SymbolAnnotation<ModuleSymbolType>>,
    ) -> (usize, usize) {
        let mut frame_size = 0;
        let mut global_count = 0;

        for binding in annotations.filter_map(|ann| ann.binding.as_ref()) {
            match binding.kind {
                BindingKind::Local => {
                    frame_size = frame_size.max(binding.slot + 1);
                }
                BindingKind::Global => {
                    global_count = global_count.max(binding.slot + 1);
                }
                BindingKind::Upvalue => {
                    // Upvalues live in an outer frame; they do not affect this frame's locals.
                }
            }
        }

        (frame_size, global_count)
    }
}
