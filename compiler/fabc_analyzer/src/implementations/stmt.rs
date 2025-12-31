use fabc_parser::ast::stmt::expr::ExprStmt;

use crate::{error::Error, reachability::Reachability, Analyzable, Analyzer};

impl Analyzable for ExprStmt {
    fn analyze(&self, analyzer: &mut Analyzer) -> Result<(), Error> {
        analyzer.set_reachability_if_absent(
            self.id,
            Reachability {
                is_reachable: true,
                terminates_normally: true,
            },
        );
        Ok(())
    }
}
