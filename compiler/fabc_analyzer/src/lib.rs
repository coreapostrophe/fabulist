use std::collections::HashMap;

use fabc_error::Error;
use fabc_parser::Parsable;

use crate::{
    annotations::SymbolAnnotation,
    symbol_table::{SymbolTable, SymbolType},
};

pub mod annotations;
pub mod data_type;
pub mod implementations;
pub mod reachability;
pub mod symbol_table;

#[derive(Default)]
pub struct AnalysisResult {
    pub symbol_type: Option<SymbolType>,
}

pub trait Analyzable {
    fn analyze(&self, analyzer: &mut Analyzer) -> AnalysisResult;
}

pub struct AnalyzerResult {
    pub symbol_annotations: HashMap<usize, SymbolAnnotation>,
    pub errors: Vec<Error>,
}

pub struct Annotation {
    pub node_id: usize,
    pub symbol_annotation: Option<SymbolAnnotation>,
}

#[derive(Default)]
pub struct Analyzer {
    symbol_table: SymbolTable,
    symbol_annotations: HashMap<usize, SymbolAnnotation>,
    errors: Vec<Error>,
}

impl Analyzer {
    pub fn analyze_ast<T>(ast: &T) -> Result<Self, Error>
    where
        T: Parsable + Analyzable,
    {
        let mut analyzer = Self::default();
        ast.analyze(&mut analyzer);
        Ok(analyzer)
    }

    pub(crate) fn symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }

    pub(crate) fn mut_symbol_table(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table
    }

    #[allow(unused)]
    pub(crate) fn annotate(&mut self, annotations: Annotation) {
        if let Some(symbol_annotation) = annotations.symbol_annotation {
            self.symbol_annotations
                .insert(annotations.node_id, symbol_annotation);
        }
    }

    #[allow(unused)]
    pub(crate) fn push_error(&mut self, error: Error) {
        self.errors.push(error);
    }
}
