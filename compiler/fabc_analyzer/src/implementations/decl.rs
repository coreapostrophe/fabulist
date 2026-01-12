#![allow(unused)]

use fabc_error::{kind::ErrorKind, Error};
use fabc_parser::ast::decl::{object::ObjectDecl, quote::QuoteDecl};

use crate::{
    types::{DataType, Field, ModuleSymbolType},
    AnalysisResult, Analyzable,
};

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
        let mut fields: Vec<Field> = Vec::new();
        for (key, value_expr) in self.map.iter() {
            let expr_val = {
                let Some(sym_type) = value_expr.analyze(analyzer).mod_sym_type else {
                    analyzer.push_error(Error::new(
                        ErrorKind::TypeInference,
                        value_expr.info().span.clone(),
                    ));
                    continue;
                };
                sym_type
            };
            fields.push(Field {
                name: key.clone(),
                r#type: Box::new(expr_val),
            });
        }

        AnalysisResult {
            mod_sym_type: Some(ModuleSymbolType::Data(DataType::Record { fields })),
        }
    }
}
