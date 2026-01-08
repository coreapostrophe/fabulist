#![allow(unused)]

use fabc_parser::ast::init::story::{
    metadata::Metadata,
    part::{
        element::{
            dialogue::DialogueElement, narration::NarrationElement, selection::SelectionElement,
            Element,
        },
        Part,
    },
    StoryInit,
};

use crate::{
    annotations::SymbolAnnotation, symbol_table::SymbolType, AnalysisResult, Analyzable, Annotation,
};

impl Analyzable for StoryInit {
    fn analyze(&self, analyzer: &mut crate::Analyzer) -> AnalysisResult {
        if let Some(metadata) = &self.metadata {
            metadata.analyze(analyzer);
        }
        self.parts.iter().for_each(|part| {
            part.analyze(analyzer);
        });

        AnalysisResult::default()
    }
}

impl Analyzable for Metadata {
    fn analyze(&self, analyzer: &mut crate::Analyzer) -> AnalysisResult {
        self.object.analyze(analyzer);

        AnalysisResult::default()
    }
}

impl Analyzable for Part {
    fn analyze(&self, analyzer: &mut crate::Analyzer) -> AnalysisResult {
        let r#type = SymbolType::Part;
        let scope_level = analyzer.symbol_table().current_level();

        analyzer
            .mut_symbol_table()
            .insert_symbol(&self.ident, r#type.clone());

        self.elements.iter().for_each(|element| {
            element.analyze(analyzer);
        });

        analyzer.annotate(Annotation {
            node_id: self.info.id,
            symbol_annotation: Some(SymbolAnnotation {
                r#type,
                scope_level,
            }),
        });

        AnalysisResult::default()
    }
}

impl Analyzable for Element {
    fn analyze(&self, analyzer: &mut crate::Analyzer) -> AnalysisResult {
        match self {
            Element::Dialogue(dialogue) => {
                dialogue.analyze(analyzer);
            }
            Element::Selection(selection) => {
                selection.analyze(analyzer);
            }
            Element::Narration(narration) => {
                narration.analyze(analyzer);
            }
        }

        AnalysisResult::default()
    }
}

impl Analyzable for DialogueElement {
    fn analyze(&self, analyzer: &mut crate::Analyzer) -> AnalysisResult {
        self.quotes.iter().for_each(|quote| {
            quote.analyze(analyzer);
        });

        AnalysisResult::default()
    }
}

impl Analyzable for SelectionElement {
    fn analyze(&self, analyzer: &mut crate::Analyzer) -> AnalysisResult {
        self.choices.iter().for_each(|choice| {
            choice.analyze(analyzer);
        });

        AnalysisResult::default()
    }
}

impl Analyzable for NarrationElement {
    fn analyze(&self, analyzer: &mut crate::Analyzer) -> AnalysisResult {
        self.quote.analyze(analyzer);

        AnalysisResult::default()
    }
}
