use std::collections::{HashMap, HashSet};

use fabc_error::{kind::CompileErrorKind, Error};
use fabc_parser::ast::{
    expr::{literal::Literal, primitive::Primitive, Expr, Primary},
    init::story::StoryInit,
};

use crate::Analyzer;

#[derive(Default)]
pub(crate) struct StoryReachability {
    start_part: Option<String>,
    declared_parts: HashSet<String>,
    current_part: Option<String>,
    edges: HashMap<String, HashSet<String>>,
    has_dynamic_target: bool,
}

impl StoryReachability {
    pub(crate) fn new(start_part: Option<String>, declared_parts: HashSet<String>) -> Self {
        Self {
            start_part,
            declared_parts,
            current_part: None,
            edges: HashMap::new(),
            has_dynamic_target: false,
        }
    }

    pub(crate) fn set_current_part(&mut self, part: Option<String>) {
        self.current_part = part;
    }

    pub(crate) fn record_target(&mut self, target: Option<String>) {
        let Some(current_part) = self.current_part.clone() else {
            return;
        };

        match target {
            Some(target) => {
                self.edges.entry(current_part).or_default().insert(target);
            }
            None => {
                self.has_dynamic_target = true;
            }
        }
    }
}

pub(crate) fn record_story_target(expr: &Expr, analyzer: &mut Analyzer) {
    let target = {
        let Some(reachability) = analyzer.story_reachability() else {
            return;
        };

        extract_part_reference(expr, &reachability.declared_parts)
    };

    analyzer.record_story_target_reference(target);
}

pub(crate) fn report_unreachable_parts(story: &StoryInit, analyzer: &mut Analyzer) {
    let Some(reachability) = analyzer.take_story_reachability() else {
        return;
    };

    let Some(start_part) = reachability.start_part else {
        return;
    };

    if !reachability.declared_parts.contains(start_part.as_str()) || reachability.has_dynamic_target
    {
        return;
    }

    let reachable_parts = collect_reachable_parts(
        &start_part,
        &reachability.edges,
        &reachability.declared_parts,
    );

    for part in &story.parts {
        if !reachable_parts.contains(part.ident.as_str()) {
            analyzer.push_warning(Error::new(
                CompileErrorKind::UnreachablePart {
                    part: part.ident.clone(),
                },
                part.info.span.clone(),
            ));
        }
    }
}

pub(crate) fn extract_start_part(story: &StoryInit) -> Option<String> {
    let metadata = story.metadata.as_ref()?;
    let start = metadata.object.map.get("start")?;
    extract_start_reference(start)
}

fn extract_start_reference(expr: &Expr) -> Option<String> {
    match expr {
        Expr::Primary {
            value: Primary::Literal(Literal::String { value, .. }),
            ..
        } => Some(value.clone()),
        Expr::Primary {
            value: Primary::Primitive(Primitive::StoryIdentifier { name, .. }),
            ..
        } => Some(name.clone()),
        Expr::Grouping { expression, .. } => extract_start_reference(expression),
        Expr::Primary {
            value: Primary::Primitive(Primitive::Grouping { expr, .. }),
            ..
        } => extract_start_reference(expr),
        _ => None,
    }
}

fn collect_reachable_parts(
    start_part: &str,
    edges: &HashMap<String, HashSet<String>>,
    known_parts: &HashSet<String>,
) -> HashSet<String> {
    let mut reachable = HashSet::new();
    let mut pending = vec![start_part.to_string()];

    while let Some(part_name) = pending.pop() {
        if !reachable.insert(part_name.clone()) {
            continue;
        }

        let Some(targets) = edges.get(part_name.as_str()) else {
            continue;
        };

        for target in targets {
            if known_parts.contains(target.as_str()) {
                pending.push(target.clone());
            }
        }
    }

    reachable
}

fn extract_part_reference(expr: &Expr, known_parts: &HashSet<String>) -> Option<String> {
    match expr {
        Expr::Primary {
            value: Primary::Primitive(Primitive::StoryIdentifier { name, .. }),
            ..
        } => Some(name.clone()),
        Expr::Primary {
            value: Primary::Primitive(Primitive::Identifier { name, .. }),
            ..
        } if known_parts.contains(name.as_str()) => Some(name.clone()),
        Expr::Primary {
            value: Primary::Literal(Literal::String { value, .. }),
            ..
        } if known_parts.contains(value.as_str()) => Some(value.clone()),
        Expr::Grouping { expression, .. } => extract_part_reference(expression, known_parts),
        Expr::Primary {
            value: Primary::Primitive(Primitive::Grouping { expr, .. }),
            ..
        } => extract_part_reference(expr, known_parts),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use fabc_error::kind::{CompileErrorKind, ErrorKind};
    use fabc_parser::{ast::init::story::StoryInit, Parser};

    use crate::Analyzer;

    #[test]
    fn story_init_reports_unreachable_parts() {
        let story = Parser::parse_ast_str::<StoryInit>(
            r#"
            Story { start: "intro" }

            # intro
            * "Begin." {
                next: () => {
                    goto connected;
                }
            }

            # connected
            * "Reached the connected part."

            # dangling
            * "This part is unreachable."
            "#,
        )
        .expect("parse story");

        let analyzer = Analyzer::analyze_ast(&story).expect("analyze failed");

        assert!(analyzer.warnings.iter().any(|warning| matches!(
            &warning.kind,
            ErrorKind::Compile(CompileErrorKind::UnreachablePart { part }) if part == "dangling"
        )));

        assert!(!analyzer.errors.iter().any(|error| matches!(
            &error.kind,
            ErrorKind::Compile(CompileErrorKind::UnreachablePart { .. })
        )));
    }

    #[test]
    fn story_init_skips_unreachable_part_diagnostics_when_goto_is_dynamic() {
        let story = Parser::parse_ast_str::<StoryInit>(
            r#"
            Story { start: "intro" }

            # intro
            * "Begin." {
                next: () => {
                    let target = context.target;
                    goto target;
                }
            }

            # connected
            * "Reached the connected part."

            # dangling
            * "This part might be reached dynamically."
            "#,
        )
        .expect("parse story");

        let analyzer = Analyzer::analyze_ast(&story).expect("analyze failed");

        assert!(!analyzer.warnings.iter().any(|warning| matches!(
            &warning.kind,
            ErrorKind::Compile(CompileErrorKind::UnreachablePart { .. })
        )));
    }
}
