use fabc_error::{
    kind::{CompileErrorKind, InternalErrorKind},
    Error,
};
use fabc_parser::ast::stmt::{
    block::BlockStmt,
    expr::ExprStmt,
    goto::GotoStmt,
    r#if::{ElseClause, IfStmt},
    r#let::LetStmt,
    r#return::ReturnStmt,
    Stmt,
};

use crate::{
    types::{DataType, ModuleSymbolType, StorySymbolType},
    AnalysisResult, Analyzable, Analyzer,
};

impl Analyzable for Stmt {
    fn analyze(&self, analyzer: &mut Analyzer) -> AnalysisResult {
        match self {
            Stmt::Block(block_stmt) => block_stmt.analyze(analyzer),
            Stmt::Expr(expr_stmt) => expr_stmt.analyze(analyzer),
            Stmt::Goto(goto_stmt) => goto_stmt.analyze(analyzer),
            Stmt::If(if_stmt) => if_stmt.analyze(analyzer),
            Stmt::Let(let_stmt) => let_stmt.analyze(analyzer),
            Stmt::Return(return_stmt) => return_stmt.analyze(analyzer),
        }
    }
}

impl Analyzable for BlockStmt {
    fn analyze(&self, analyzer: &mut Analyzer) -> AnalysisResult {
        analyzer.mut_mod_sym_table().enter_scope();

        let mut return_type: Option<ModuleSymbolType> = None;

        for statement in &self.statements {
            match statement {
                Stmt::Return(return_statement) => {
                    let analyzed_return = return_statement.analyze(analyzer);
                    if let Some(ret_type) = analyzed_return.mod_sym_type {
                        return_type = Some(ret_type);
                    }
                }
                _ => {
                    statement.analyze(analyzer);
                }
            }
        }

        analyzer.mut_mod_sym_table().exit_scope();

        AnalysisResult {
            mod_sym_type: return_type,
            ..Default::default()
        }
    }
}

impl Analyzable for ExprStmt {
    fn analyze(&self, analyzer: &mut Analyzer) -> AnalysisResult {
        self.expr.analyze(analyzer);

        AnalysisResult::default()
    }
}

impl Analyzable for GotoStmt {
    fn analyze(&self, analyzer: &mut Analyzer) -> AnalysisResult {
        let target_type = {
            let Some(symbol) = self.target.analyze(analyzer).story_sym_type else {
                analyzer.push_error(Error::new(
                    CompileErrorKind::TypeInference,
                    self.info.span.clone(),
                ));
                return AnalysisResult::default();
            };
            symbol.clone()
        };

        if !matches!(target_type, StorySymbolType::Part) {
            analyzer.push_error(Error::new(
                CompileErrorKind::ExpectedType {
                    expected: "part".to_string(),
                    found: format!("{}", target_type),
                },
                self.info.span.clone(),
            ));
        }

        AnalysisResult::default()
    }
}

impl Analyzable for IfStmt {
    fn analyze(&self, analyzer: &mut Analyzer) -> AnalysisResult {
        self.condition.analyze(analyzer);
        self.then_branch.analyze(analyzer);

        if let Some(else_branch) = &self.else_branch {
            match else_branch {
                ElseClause::If(if_stmt) => {
                    if_stmt.analyze(analyzer);
                }
                ElseClause::Block(block_stmt) => {
                    block_stmt.analyze(analyzer);
                }
            }
        }

        AnalysisResult::default()
    }
}

impl Analyzable for LetStmt {
    fn analyze(&self, analyzer: &mut Analyzer) -> AnalysisResult {
        let Some(var_type) = self.initializer.analyze(analyzer).mod_sym_type else {
            analyzer.push_error(Error::new(
                CompileErrorKind::TypeInference,
                self.info.span.clone(),
            ));
            return AnalysisResult::default();
        };

        let var_symbol = {
            let Some(symbol) = analyzer
                .mut_mod_sym_table()
                .assign_symbol(&self.name, var_type.clone())
            else {
                analyzer.push_error(Error::new(
                    InternalErrorKind::InvalidAssignment,
                    self.info.span.clone(),
                ));
                return AnalysisResult::default();
            };
            symbol.clone()
        };

        analyzer.annotate_mod_symbol(self.info.id, var_symbol.clone().into());

        AnalysisResult::default()
    }
}

impl Analyzable for ReturnStmt {
    fn analyze(&self, analyzer: &mut Analyzer) -> AnalysisResult {
        if let Some(return_expr) = &self.value {
            return_expr.analyze(analyzer)
        } else {
            AnalysisResult {
                mod_sym_type: Some(ModuleSymbolType::Data(DataType::None)),
                ..Default::default()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{info, number_expr, story_identifier_expr};

    #[test]
    fn let_stmt_binds_symbol_and_annotation() {
        let mut analyzer = Analyzer::default();

        let let_stmt = LetStmt {
            info: info(50),
            name: "a".to_string(),
            initializer: number_expr(51, 7.0),
        };

        let_stmt.analyze(&mut analyzer);

        let sym = analyzer
            .mut_mod_sym_table()
            .lookup_symbol("a")
            .expect("symbol missing")
            .r#type
            .clone();

        assert_eq!(sym, ModuleSymbolType::Data(DataType::Number));

        let annotation = analyzer
            .mod_sym_annotations
            .get(&50)
            .expect("annotation missing");
        assert_eq!(annotation.name.as_deref(), Some("a"));
        assert_eq!(annotation.r#type, ModuleSymbolType::Data(DataType::Number));
    }

    #[test]
    fn block_returns_inner_return_type() {
        let return_stmt = Stmt::Return(ReturnStmt {
            info: info(60),
            value: Some(number_expr(61, 3.0)),
        });

        let block = BlockStmt {
            info: info(62),
            first_return: Some(0),
            statements: vec![return_stmt],
        };

        let result = block.analyze(&mut Analyzer::default());

        assert_eq!(
            result.mod_sym_type,
            Some(ModuleSymbolType::Data(DataType::Number))
        );
    }

    #[test]
    fn goto_requires_part_symbol() {
        let mut analyzer = Analyzer::default();
        analyzer
            .mut_story_sym_table()
            .assign_symbol("speaker", StorySymbolType::Speaker);

        let goto = GotoStmt {
            info: info(70),
            target: Box::new(story_identifier_expr(71, "speaker")),
        };

        goto.analyze(&mut analyzer);

        let kinds: Vec<_> = analyzer.errors.iter().map(|e| e.kind.clone()).collect();
        assert!(
            kinds.iter().any(|k| matches!(
                k,
                fabc_error::kind::ErrorKind::Compile(
                    fabc_error::kind::CompileErrorKind::TypeInference
                )
            )),
            "unexpected error kinds: {:?}",
            kinds
        );
    }
}
