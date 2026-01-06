use fabc_error::Error;
use fabc_parser::Parsable;

use crate::{data_type::DataType, symbol_table::SymbolTable};

pub mod data_type;
pub mod implementations;
pub mod reachability;
pub mod symbol_table;

#[derive(Default)]
pub struct AnalysisResult {
    pub data_type: Option<DataType>,
}

pub trait Analyzable {
    fn analyze(&self, analyzer: &mut Analyzer) -> AnalysisResult;
}

pub struct Analyzer {
    symbol_table: SymbolTable,
    errors: Vec<Error>,
}

impl Analyzer {
    pub fn analyze<T>(ast: &T) -> Result<Self, Error>
    where
        T: Parsable + Analyzable,
    {
        let mut analyzer = Self {
            symbol_table: SymbolTable::new(),
            errors: Vec::new(),
        };

        ast.analyze(&mut analyzer);

        Ok(analyzer)
    }
    pub fn symbol_table(&self) -> &SymbolTable {
        &self.symbol_table
    }
    pub fn mut_symbol_table(&mut self) -> &mut SymbolTable {
        &mut self.symbol_table
    }
    pub fn errors(&self) -> &[Error] {
        &self.errors
    }
    pub(crate) fn _push_error(&mut self, error: Error) {
        self.errors.push(error);
    }
}
