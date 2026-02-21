use fabc_error::{kind::CompileErrorKind, Error};
use fabc_parser::ast::decl::{object::ObjectDecl, quote::QuoteDecl};

use crate::{
    types::{DataType, Field, ModuleSymbolType, SymbolAnnotation},
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
                        CompileErrorKind::TypeInference,
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

        let object_sym_type = ModuleSymbolType::Data(DataType::Record { fields });

        analyzer.annotate_mod_symbol(
            self.info.id,
            SymbolAnnotation {
                name: None,
                r#type: object_sym_type.clone(),
                binding: None,
            },
        );

        AnalysisResult {
            mod_sym_type: Some(object_sym_type),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        test_utils::{info, number_expr, string_expr},
        Analyzer,
    };

    #[test]
    fn object_decl_turns_into_record_type() {
        let mut map = std::collections::BTreeMap::new();
        map.insert("x".to_string(), number_expr(2, 1.0));
        map.insert("name".to_string(), string_expr(3, "foo"));

        let object = ObjectDecl {
            info: info(10),
            map,
        };

        let analyzer = Analyzer::analyze_ast(&object).expect("analyze failed");

        let annotation = analyzer
            .mod_sym_annotations
            .get(&10)
            .expect("annotation missing");

        match &annotation.r#type {
            ModuleSymbolType::Data(DataType::Record { fields }) => {
                assert_eq!(fields.len(), 2);
                assert!(fields.iter().any(
                    |f| f.name == "x" && *f.r#type == ModuleSymbolType::Data(DataType::Number)
                ));
                assert!(fields
                    .iter()
                    .any(|f| f.name == "name"
                        && *f.r#type == ModuleSymbolType::Data(DataType::String)));
            }
            other => panic!("unexpected annotation: {other}"),
        }
    }
}
