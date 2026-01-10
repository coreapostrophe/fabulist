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

use crate::{symbol_table::Symbol, types::StorySymbolType, AnalysisResult, Analyzable};

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
        let part_name = self.ident.clone();
        let part_type = StorySymbolType::Part;
        let part_sl = analyzer.story_sym_table().current_level();

        analyzer
            .mut_story_sym_table()
            .insert_symbol(&self.ident, part_type.clone());

        self.elements.iter().for_each(|element| {
            element.analyze(analyzer);
        });

        analyzer.annotate_story_symbol(
            self.info.id,
            Symbol {
                name: part_name.clone(),
                r#type: part_type.clone(),
                scope_level: part_sl,
            },
        );

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
        let speaker_name = self.speaker.clone();
        let speaker_type = StorySymbolType::Speaker;
        let speaker_sl = analyzer.story_sym_table().current_level();

        analyzer
            .mut_story_sym_table()
            .insert_symbol(&self.speaker, speaker_type.clone());

        self.quotes.iter().for_each(|quote| {
            quote.analyze(analyzer);
        });

        analyzer.annotate_story_symbol(
            self.info.id,
            Symbol {
                name: speaker_name,
                r#type: speaker_type,
                scope_level: speaker_sl,
            },
        );

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
