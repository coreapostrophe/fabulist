use std::collections::HashMap;

use fabc_error::Error;
use fabc_parser::ast::init::Init;
#[cfg(test)]
use fabc_parser::Parsable;

use crate::{
    symbol_table::SymbolTable,
    types::{ModuleSymbolType, StorySymbolType, SymbolAnnotation},
};

pub mod implementations;
pub mod reachability;
pub mod symbol_table;
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
}

#[derive(Default)]
pub struct Analyzer {
    story_sym_table: SymbolTable<StorySymbolType>,
    mod_sym_table: SymbolTable<ModuleSymbolType>,
    story_sym_annotations: HashMap<usize, SymbolAnnotation<StorySymbolType>>,
    mod_sym_annotations: HashMap<usize, SymbolAnnotation<ModuleSymbolType>>,
    errors: Vec<Error>,
}

impl Analyzer {
    pub fn analyze(inits: Vec<Init>) -> AnalyzerResult {
        let mut analyzer = Self::default();

        for init in &inits {
            init.analyze(&mut analyzer);
        }

        AnalyzerResult {
            story_sym_annotations: analyzer.story_sym_annotations,
            mod_sym_annotations: analyzer.mod_sym_annotations,
            errors: analyzer.errors,
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

    #[allow(unused)]
    pub(crate) fn push_error(&mut self, error: Error) {
        self.errors.push(error);
    }
}
