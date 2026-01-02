use fabc_parser::ast::init::{
    module::ModuleInit,
    story::{
        metadata::Metadata,
        part::{
            element::{dialogue::Dialogue, narration::Narration, selection::Selection, Element},
            Part,
        },
        StoryInit,
    },
};

use crate::{Analyzable, Analyzer};

impl Analyzable for ModuleInit {
    fn analyze(&self, _analyzer: &mut Analyzer) {}
}

impl Analyzable for StoryInit {
    fn analyze(&self, analyzer: &mut Analyzer) {
        if let Some(metadata) = &self.metadata {
            metadata.analyze(analyzer);
        }
        self.parts.iter().for_each(|part| part.analyze(analyzer));
    }
}

impl Analyzable for Metadata {
    fn analyze(&self, analyzer: &mut Analyzer) {
        self.object.analyze(analyzer);
    }
}

impl Analyzable for Part {
    fn analyze(&self, analyzer: &mut Analyzer) {
        self.elements
            .iter()
            .for_each(|element| element.analyze(analyzer));
    }
}

impl Analyzable for Element {
    fn analyze(&self, analyzer: &mut Analyzer) {
        match self {
            Element::Dialogue(dialogue_element) => {
                dialogue_element.analyze(analyzer);
            }
            Element::Narration(narration_element) => {
                narration_element.analyze(analyzer);
            }
            Element::Selection(selection_element) => {
                selection_element.analyze(analyzer);
            }
        }
    }
}

impl Analyzable for Dialogue {
    fn analyze(&self, analyzer: &mut Analyzer) {
        self.quotes.iter().for_each(|quote| quote.analyze(analyzer));
    }
}

impl Analyzable for Narration {
    fn analyze(&self, analyzer: &mut Analyzer) {
        self.quote.analyze(analyzer);
    }
}

impl Analyzable for Selection {
    fn analyze(&self, analyzer: &mut Analyzer) {
        self.choices
            .iter()
            .for_each(|choice| choice.analyze(analyzer));
    }
}
