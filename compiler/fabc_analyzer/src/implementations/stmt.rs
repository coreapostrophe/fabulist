#![allow(unused)]
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
    symbol_table::Symbol,
    types::{DataType, ModuleSymbolType},
    AnalysisResult, Analyzable, Analyzer,
};

impl Analyzable for Stmt {
    fn analyze(&self, analyzer: &mut Analyzer) -> AnalysisResult {
        todo!()
    }
}

impl Analyzable for BlockStmt {
    fn analyze(&self, analyzer: &mut Analyzer) -> AnalysisResult {
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
        let var_name = self.name.clone();
        let var_type = self
            .initializer
            .analyze(analyzer)
            .mod_sym_type
            .unwrap_or(ModuleSymbolType::Data(DataType::None));
        let var_sl = analyzer.mod_sym_table().current_level();

        analyzer
            .mut_mod_sym_table()
            .insert_symbol(&self.name, var_type.clone());

        analyzer.annotate_mod_symbol(
            self.info.id,
            Symbol {
                name: var_name,
                r#type: var_type,
                scope_level: var_sl,
            },
        );

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
