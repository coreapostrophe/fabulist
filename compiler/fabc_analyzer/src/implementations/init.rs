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

use crate::{error::Error, Analyzable, Analyzer};

impl Analyzable for ModuleInit {
    fn analyze(&self, _analyzer: &mut Analyzer) -> Result<(), Error> {
        Ok(())
    }
}

impl Analyzable for StoryInit {
    fn analyze(&self, analyzer: &mut Analyzer) -> Result<(), Error> {
        if let Some(metadata) = &self.metadata {
            metadata.analyze(analyzer)?;
        }
        self.parts
            .iter()
            .try_for_each(|part| part.analyze(analyzer))?;
        Ok(())
    }
}

impl Analyzable for Metadata {
    fn analyze(&self, analyzer: &mut Analyzer) -> Result<(), Error> {
        self.object.analyze(analyzer)?;
        Ok(())
    }
}

impl Analyzable for Part {
    fn analyze(&self, analyzer: &mut Analyzer) -> Result<(), Error> {
        self.elements
            .iter()
            .try_for_each(|element| element.analyze(analyzer))?;
        Ok(())
    }
}

impl Analyzable for Element {
    fn analyze(&self, analyzer: &mut Analyzer) -> Result<(), Error> {
        match self {
            Element::Dialogue(dialogue_element) => {
                dialogue_element.analyze(analyzer)?;
            }
            Element::Narration(narration_element) => {
                narration_element.analyze(analyzer)?;
            }
            Element::Selection(selection_element) => {
                selection_element.analyze(analyzer)?;
            }
        }
        Ok(())
    }
}

impl Analyzable for Dialogue {
    fn analyze(&self, analyzer: &mut Analyzer) -> Result<(), Error> {
        self.quotes
            .iter()
            .try_for_each(|quote| quote.analyze(analyzer))?;
        Ok(())
    }
}

impl Analyzable for Narration {
    fn analyze(&self, analyzer: &mut Analyzer) -> Result<(), Error> {
        self.quote.analyze(analyzer)?;
        Ok(())
    }
}

impl Analyzable for Selection {
    fn analyze(&self, analyzer: &mut Analyzer) -> Result<(), Error> {
        self.choices
            .iter()
            .try_for_each(|choice| choice.analyze(analyzer))?;
        Ok(())
    }
}
