use fabc_parser::ast::stmt::{
    block::BlockStmt, expr::ExprStmt, goto::GotoStmt, r#if::IfStmt, r#let::LetStmt, Stmt,
};

use crate::{Analyzable, Analyzer};

impl Analyzable for Stmt {
    fn analyze(&self, analyzer: &mut Analyzer) {
        match self {
            Stmt::Expr(expr_stmt) => expr_stmt.analyze(analyzer),
            Stmt::Block(block_stmt) => block_stmt.analyze(analyzer),
            _ => todo!(),
        }
    }
}

impl Analyzable for ExprStmt {
    fn analyze(&self, analyzer: &mut Analyzer) {
        analyzer.set_reachability(self.id);
    }
}

impl Analyzable for BlockStmt {
    fn analyze(&self, analyzer: &mut Analyzer) {
        analyzer.set_reachability(self.id).set_is_reachable(true);

        let reachable_first_statement = analyzer
            .get_reachability(self.id)
            .filter(|reachability| reachability.is_reachable())
            .and_then(|_| self.statements.first());

        if let Some(statement) = reachable_first_statement {
            analyzer
                .set_reachability(statement.id())
                .set_is_reachable(true);
        }
    }
}

impl Analyzable for GotoStmt {
    fn analyze(&self, analyzer: &mut Analyzer) {
        analyzer.set_reachability(self.id);
    }
}

impl Analyzable for IfStmt {
    fn analyze(&self, analyzer: &mut Analyzer) {
        analyzer.set_reachability(self.id);
    }
}

impl Analyzable for LetStmt {
    fn analyze(&self, analyzer: &mut Analyzer) {
        analyzer.set_reachability(self.id);
    }
}
