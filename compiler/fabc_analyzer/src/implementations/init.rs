use fabc_error::{kind::InternalErrorKind, Error};
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
    types::{ModuleSymbolType, StorySymbolType},
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
                        InternalErrorKind::InvalidAssignment,
                        self.info.span.clone(),
                    ));
                    return AnalysisResult::default();
                };
                symbol.clone()
            };

            analyzer.annotate_mod_symbol(self.info.id, alias_symbol.into());

            return AnalysisResult {
                mod_sym_type: Some(sym_type),
                ..Default::default()
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
                    InternalErrorKind::InvalidAssignment,
                    self.info.span.clone(),
                ));
                return AnalysisResult::default();
            };
            symbol.clone()
        };

        self.elements.iter().for_each(|element| {
            element.analyze(analyzer);
        });

        analyzer.annotate_story_symbol(self.info.id, part_symbol.into());

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
                    InternalErrorKind::InvalidAssignment,
                    self.info.span.clone(),
                ));
                return AnalysisResult::default();
            };
            symbol.clone()
        };

        self.quotes.iter().for_each(|quote| {
            quote.analyze(analyzer);
        });

        analyzer.annotate_story_symbol(self.info.id, speaker_symbol.into());

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{info, string_expr};
    use crate::Analyzer;
    use fabc_parser::ast::{
        decl::quote::QuoteDecl,
        init::story::{
            metadata::Metadata,
            part::{
                element::{dialogue::DialogueElement, narration::NarrationElement, Element},
                Part,
            },
            StoryInit,
        },
        init::Init,
    };

    #[test]
    fn module_init_alias_is_annotated() {
        let init = ModuleInit {
            info: info(1),
            path: "path/to/module".to_string(),
            alias: Some("utils".to_string()),
        };

        let analyzer = Analyzer::analyze_ast(&init).expect("analyze failed");

        let annotation = analyzer
            .mod_sym_annotations
            .get(&1)
            .expect("annotation missing");

        assert_eq!(annotation.name.as_deref(), Some("utils"));
        assert_eq!(
            annotation.r#type,
            ModuleSymbolType::Module {
                name: "utils".to_string(),
            }
        );
    }

    #[test]
    fn part_and_dialogue_symbols_are_tracked() {
        let part = Part {
            info: info(80),
            ident: "intro".to_string(),
            elements: vec![Element::Dialogue(DialogueElement {
                info: info(81),
                speaker: "guide".to_string(),
                quotes: vec![QuoteDecl {
                    info: info(82),
                    text: "hi".to_string(),
                    properties: None,
                }],
            })],
        };

        let mut analyzer = Analyzer::default();
        part.analyze(&mut analyzer);

        let part_annotation = analyzer
            .story_sym_annotations
            .get(&80)
            .expect("part annotation missing");
        assert_eq!(part_annotation.r#type, StorySymbolType::Part);

        let speaker_annotation = analyzer
            .story_sym_annotations
            .get(&81)
            .expect("speaker annotation missing");
        assert_eq!(speaker_annotation.r#type, StorySymbolType::Speaker);
    }

    #[test]
    fn story_init_traverses_metadata_and_parts() {
        let mut metadata_map = std::collections::BTreeMap::new();
        metadata_map.insert("title".to_string(), string_expr(90, "Story"));

        let story = StoryInit {
            info: info(91),
            metadata: Some(Metadata {
                info: info(92),
                object: fabc_parser::ast::decl::object::ObjectDecl {
                    info: info(93),
                    map: metadata_map,
                },
            }),
            parts: vec![Part {
                info: info(94),
                ident: "p1".to_string(),
                elements: vec![Element::Narration(NarrationElement {
                    info: info(95),
                    quote: QuoteDecl {
                        info: info(96),
                        text: "hello".to_string(),
                        properties: None,
                    },
                })],
            }],
        };

        let analyzer = Analyzer::analyze_ast(&Init::Story(story)).expect("analyze failed");

        assert!(analyzer.story_sym_annotations.contains_key(&94));
        assert!(analyzer.mod_sym_annotations.contains_key(&93));
    }
}
