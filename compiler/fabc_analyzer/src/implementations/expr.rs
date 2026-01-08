use fabc_parser::ast::expr::{literal::Literal, primitive::Primitive};

use crate::{data_type::DataType, AnalysisResult, Analyzable};

impl Analyzable for Literal {
    fn analyze(&self, _analyzer: &mut crate::Analyzer) -> AnalysisResult {
        let data_type = match self {
            Literal::Number(_) => DataType::Number,
            Literal::String(_) => DataType::String,
            Literal::Boolean(_) => DataType::Boolean,
            Literal::None => DataType::None,
        };

        AnalysisResult {
            data_type: Some(data_type),
        }
    }
}

impl Analyzable for Primitive {
    fn analyze(&self, _analyzer: &mut crate::Analyzer) -> AnalysisResult {
        todo!()
    }
}
