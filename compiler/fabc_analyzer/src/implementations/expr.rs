#![allow(unused)]
use fabc_parser::ast::expr::{literal::Literal, primitive::Primitive, Expr};

use crate::{
    types::{DataType, ModuleSymbolType},
    AnalysisResult, Analyzable,
};

impl Analyzable for Expr {
    fn analyze(&self, analyzer: &mut crate::Analyzer) -> AnalysisResult {
        todo!()
    }
}

impl Analyzable for Literal {
    fn analyze(&self, _analyzer: &mut crate::Analyzer) -> AnalysisResult {
        let data_type = match self {
            Literal::Number(_) => DataType::Number,
            Literal::String(_) => DataType::String,
            Literal::Boolean(_) => DataType::Boolean,
            Literal::None => DataType::None,
        };

        AnalysisResult {
            mod_sym_type: Some(ModuleSymbolType::Data(data_type)),
        }
    }
}

impl Analyzable for Primitive {
    fn analyze(&self, analyzer: &mut crate::Analyzer) -> AnalysisResult {
        todo!()
    }
}
