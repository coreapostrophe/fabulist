use fabc_error::{kind::ErrorKind, Error};
use fabc_parser::ast::init::{
    module::ModuleInit,
    story::{
        metadata::Metadata,
        part::{
            element::{
                dialogue::DialogueElement, narration::NarrationElement,
                selection::SelectionElement, Element,
            },
            Part,
        },
        StoryInit,
    },
    Init,
};

use crate::{
    types::{ModuleSymbolType, StorySymbolType, Symbol},
    AnalysisResult, Analyzable,
};

impl Analyzable for Init {
    fn analyze(&self, analyzer: &mut crate::Analyzer) -> AnalysisResult {
        match self {
            Init::Story(story_init) => story_init.analyze(analyzer),
            Init::Module(module_init) => module_init.analyze(analyzer),
        }
    }
}

impl Analyzable for ModuleInit {
    fn analyze(&self, analyzer: &mut crate::Analyzer) -> AnalysisResult {
        if let Some(module_alias) = self.alias.as_ref() {
            let sym_type = ModuleSymbolType::Module {
                name: module_alias.clone(),
            };

            let alias_symbol = {
                let Some(symbol) = analyzer
                    .mut_mod_sym_table()
                    .assign_symbol(module_alias, sym_type.clone())
                else {
                    analyzer.push_error(Error::new(
                        ErrorKind::InternalAssignment,
                        self.info.span.clone(),
                    ));
                    return AnalysisResult::default();
                };
                symbol.clone()
            };

            analyzer.annotate_mod_symbol(self.info.id, alias_symbol);

            return AnalysisResult {
                mod_sym_type: Some(sym_type),
            };
        }

        AnalysisResult::default()
    }
}

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
        let part_symbol = {
            let Some(symbol) = analyzer
                .mut_story_sym_table()
                .assign_symbol(&self.ident, StorySymbolType::Part)
            else {
                analyzer.push_error(Error::new(
                    ErrorKind::InternalAssignment,
                    self.info.span.clone(),
                ));
                return AnalysisResult::default();
            };
            symbol.clone()
        };

        self.elements.iter().for_each(|element| {
            element.analyze(analyzer);
        });

        analyzer.annotate_story_symbol(self.info.id, part_symbol);

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
        let speaker_symbol = {
            let Some(symbol) = analyzer
                .mut_story_sym_table()
                .assign_symbol(&self.speaker, StorySymbolType::Speaker)
            else {
                analyzer.push_error(Error::new(
                    ErrorKind::InternalAssignment,
                    self.info.span.clone(),
                ));
                return AnalysisResult::default();
            };
            symbol.clone()
        };

        self.quotes.iter().for_each(|quote| {
            quote.analyze(analyzer);
        });

        analyzer.annotate_story_symbol(self.info.id, speaker_symbol);

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
