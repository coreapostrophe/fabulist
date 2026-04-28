use std::collections::HashMap;

use fabc_error::Error;
use fabc_parser::ast::init::Init;
#[cfg(test)]
use fabc_parser::Parsable;

use crate::{
    reachability::StoryReachability,
    symbol_table::SymbolTable,
    types::{ModuleSymbolType, StorySymbolType, SymbolAnnotation},
};

pub mod implementations;
mod reachability;
pub mod symbol_table;
#[cfg(test)]
pub mod test_utils;
pub mod types;

#[derive(Default)]
pub struct AnalysisResult {
    pub mod_sym_type: Option<ModuleSymbolType>,
    pub story_sym_type: Option<StorySymbolType>,
}

pub trait Analyzable {
    fn analyze(&self, analyzer: &mut Analyzer) -> AnalysisResult;
}

pub struct AnalyzerResult {
    pub story_sym_annotations: HashMap<usize, SymbolAnnotation<StorySymbolType>>,
    pub mod_sym_annotations: HashMap<usize, SymbolAnnotation<ModuleSymbolType>>,
    pub errors: Vec<Error>,
    pub warnings: Vec<Error>,
}

#[derive(Default)]
pub struct Analyzer {
    story_sym_table: SymbolTable<StorySymbolType>,
    mod_sym_table: SymbolTable<ModuleSymbolType>,
    story_sym_annotations: HashMap<usize, SymbolAnnotation<StorySymbolType>>,
    mod_sym_annotations: HashMap<usize, SymbolAnnotation<ModuleSymbolType>>,
    story_reachability: Option<StoryReachability>,
    errors: Vec<Error>,
    warnings: Vec<Error>,
}

impl Analyzer {
    pub fn analyze(inits: &[Init]) -> AnalyzerResult {
        let mut analyzer = Self::default();

        for init in inits {
            init.analyze(&mut analyzer);
        }

        AnalyzerResult {
            story_sym_annotations: analyzer.story_sym_annotations,
            mod_sym_annotations: analyzer.mod_sym_annotations,
            errors: analyzer.errors,
            warnings: analyzer.warnings,
        }
    }

    #[cfg(test)]
    pub fn analyze_ast<T>(ast: &T) -> Result<Self, Error>
    where
        T: Parsable + Analyzable,
    {
        let mut analyzer = Self::default();
        ast.analyze(&mut analyzer);
        Ok(analyzer)
    }

    pub(crate) fn mut_story_sym_table(&mut self) -> &mut SymbolTable<StorySymbolType> {
        &mut self.story_sym_table
    }

    pub(crate) fn mut_mod_sym_table(&mut self) -> &mut SymbolTable<ModuleSymbolType> {
        &mut self.mod_sym_table
    }

    pub(crate) fn annotate_story_symbol(
        &mut self,
        node_id: usize,
        symbol: SymbolAnnotation<StorySymbolType>,
    ) {
        self.story_sym_annotations.insert(node_id, symbol);
    }

    pub(crate) fn annotate_mod_symbol(
        &mut self,
        node_id: usize,
        symbol: SymbolAnnotation<ModuleSymbolType>,
    ) {
        self.mod_sym_annotations.insert(node_id, symbol);
    }

    pub(crate) fn begin_story_reachability(&mut self, reachability: StoryReachability) {
        self.story_reachability = Some(reachability);
    }

    pub(crate) fn story_reachability(&self) -> Option<&StoryReachability> {
        self.story_reachability.as_ref()
    }

    pub(crate) fn set_current_story_part(&mut self, part: Option<String>) {
        if let Some(reachability) = self.story_reachability.as_mut() {
            reachability.set_current_part(part);
        }
    }

    pub(crate) fn record_story_target_reference(&mut self, target: Option<String>) {
        if let Some(reachability) = self.story_reachability.as_mut() {
            reachability.record_target(target);
        }
    }

    pub(crate) fn take_story_reachability(&mut self) -> Option<StoryReachability> {
        self.story_reachability.take()
    }

    pub(crate) fn push_error(&mut self, error: Error) {
        self.errors.push(error);
    }

    pub(crate) fn push_warning(&mut self, warning: Error) {
        self.warnings.push(warning);
    }
}
