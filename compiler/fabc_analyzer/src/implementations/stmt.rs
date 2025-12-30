use fabc_parser::ast::stmt::{expr::ExprStmt, function::FunctionStmt, module::ModuleStmt};

use crate::{error::Error, reachability::Reachability, Analyzable, Analyzer};

impl Analyzable for ModuleStmt {
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

impl Analyzable for FunctionStmt {
    fn analyze(&self, analyzer: &mut Analyzer) -> Result<(), Error> {
        analyzer.set_reachability_if_absent(
            self.id,
            Reachability {
                is_reachable: true,
                terminates_normally: true,
            },
        );

        let reachable_first_statement = analyzer
            .get_reachability(&self.id)
            .filter(|reachability| reachability.is_reachable)
            .and_then(|_| self.body.statements.first());

        if let Some(statement) = reachable_first_statement {
            analyzer.override_reachability(
                statement.id(),
                Reachability {
                    is_reachable: true,
                    terminates_normally: true,
                },
            );
        } else {
            self.body.statements.iter().for_each(|statement| {
                analyzer.override_reachability(
                    statement.id(),
                    Reachability {
                        is_reachable: false,
                        terminates_normally: false,
                    },
                );
            });
        }
        Ok(())
    }
}
