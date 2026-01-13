#![allow(unused)]
use fabc_error::{kind::ErrorKind, Error};
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
    types::{DataType, ModuleSymbolType, Symbol},
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
        self.target.analyze(analyzer);

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
            analyzer.push_error(Error::new(ErrorKind::TypeInference, self.info.span.clone()));
            return AnalysisResult::default();
        };

        let var_symbol = {
            let Some(symbol) = analyzer
                .mut_mod_sym_table()
                .assign_symbol(&self.name, var_type.clone())
            else {
                analyzer.push_error(Error::new(
                    ErrorKind::InternalAssignment,
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
            AnalysisResult::default()
        }
    }
}
