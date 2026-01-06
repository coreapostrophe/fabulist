#![allow(unused)]

use fabc_parser::ast::decl::{object::ObjectDecl, quote::QuoteDecl};

use crate::{AnalysisResult, Analyzable};

impl Analyzable for QuoteDecl {
    fn analyze(&self, analyzer: &mut crate::Analyzer) -> AnalysisResult {
        if let Some(properties) = &self.properties {
            properties.analyze(analyzer);
        }

        AnalysisResult::default()
    }
}

impl Analyzable for ObjectDecl {
    fn analyze(&self, analyzer: &mut crate::Analyzer) -> AnalysisResult {
        todo!()
    }
}
