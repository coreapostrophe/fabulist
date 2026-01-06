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

use crate::{symbol_table::SymbolType, Analyzable};

impl Analyzable for StoryInit {
    fn analyze(&self, analyzer: &mut crate::Analyzer) {
        if let Some(metadata) = &self.metadata {
            metadata.analyze(analyzer);
        }
        self.parts.iter().for_each(|part| {
            part.analyze(analyzer);
        });
    }
}

impl Analyzable for Metadata {
    fn analyze(&self, analyzer: &mut crate::Analyzer) {
        self.object.analyze(analyzer);
    }
}

impl Analyzable for Part {
    fn analyze(&self, analyzer: &mut crate::Analyzer) {
        analyzer
            .mut_symbol_table()
            .insert_symbol(&self.ident, SymbolType::Part);

        self.elements.iter().for_each(|element| {
            element.analyze(analyzer);
        });
    }
}

impl Analyzable for Element {
    fn analyze(&self, analyzer: &mut crate::Analyzer) {
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
    }
}

impl Analyzable for DialogueElement {
    fn analyze(&self, analyzer: &mut crate::Analyzer) {
        analyzer
            .mut_symbol_table()
            .insert_symbol(&self.speaker, SymbolType::Speaker);

        self.quotes.iter().for_each(|quote| {
            quote.analyze(analyzer);
        });
    }
}

impl Analyzable for SelectionElement {
    fn analyze(&self, analyzer: &mut crate::Analyzer) {
        self.choices.iter().for_each(|choice| {
            choice.analyze(analyzer);
        });
    }
}

impl Analyzable for NarrationElement {
    fn analyze(&self, analyzer: &mut crate::Analyzer) {
        self.quote.analyze(analyzer);
    }
}
