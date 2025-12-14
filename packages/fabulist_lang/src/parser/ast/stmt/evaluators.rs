//! Statement evaluators that execute within a runtime environment.
use crate::{
    error::RuntimeError,
    interpreter::environment::{Environment, RuntimeEnvironment},
    interpreter::{runtime_value::RuntimeValue, Evaluable},
    parser::ast::stmt::models::{BlockStmt, ElseClause, ExprStmt, GotoStmt, IfStmt, LetStmt, Stmt},
};

impl Evaluable for BlockStmt {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
    ) -> Self::Output {
        let block_environment = Environment::add_empty_child(environment);

        for statement in &self.statements {
            statement.evaluate(&block_environment, context)?;
        }

        Ok(RuntimeValue::None {
            span: self.span.clone(),
        })
    }
}

impl Evaluable for IfStmt {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
    ) -> Self::Output {
        let mut condition = self.condition.evaluate(environment, context)?;

        if let RuntimeValue::Identifier { name, span } = &condition {
            if let Some(value) = Environment::get_value(environment, name) {
                condition = value;
            } else {
                return Err(RuntimeError::IdentifierDoesNotExist(span.clone()));
            }
        }

        if condition.to_bool()? {
            self.block_stmt.evaluate(environment, context)
        } else if let Some(else_stmt) = &self.else_stmt {
            match else_stmt.as_ref() {
                ElseClause::If(if_stmt) => if_stmt.evaluate(environment, context),
                ElseClause::Block(block_stmt) => block_stmt.evaluate(environment, context),
            }
        } else {
            Ok(RuntimeValue::None {
                span: self.span.clone(),
            })
        }
    }
}

impl Evaluable for LetStmt {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
    ) -> Self::Output {
        let identifier = &self.identifier.evaluate(environment, context)?;
        let value = self.value.evaluate(environment, context)?;

        let RuntimeValue::Identifier { name: key, .. } = identifier else {
            return Err(RuntimeError::InvalidIdentifier(
                self.identifier.span.clone(),
            ));
        };

        Environment::insert(environment, key, value);

        Ok(RuntimeValue::None {
            span: self.span.clone(),
        })
    }
}

impl Evaluable for GotoStmt {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        _environment: &RuntimeEnvironment,
        _context: &RuntimeEnvironment,
    ) -> Self::Output {
        todo!()
    }
}

impl Evaluable for ExprStmt {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
    ) -> Self::Output {
        self.value.evaluate(environment, context)?;

        Ok(RuntimeValue::None {
            span: self.span.clone(),
        })
    }
}

impl Evaluable for Stmt {
    type Output = Result<RuntimeValue, RuntimeError>;

    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
    ) -> Self::Output {
        match self {
            Stmt::Block(block_stmt) => block_stmt.evaluate(environment, context),
            Stmt::If(if_stmt) => if_stmt.evaluate(environment, context),
            Stmt::Let(let_stmt) => let_stmt.evaluate(environment, context),
            Stmt::Goto(goto_stmt) => goto_stmt.evaluate(environment, context),
            Stmt::Expr(expr_stmt) => expr_stmt.evaluate(environment, context),
        }
    }
}

#[cfg(test)]
mod stmt_evaluators_tests {
    use crate::{
        error::OwnedSpan,
        interpreter::environment::Environment,
        interpreter::runtime_value::RuntimeValue,
        parser::ast::{
            stmt::models::{BlockStmt, ExprStmt, LetStmt},
            AssertEvaluateOptions, AstTestHelper,
        },
        parser::Rule,
    };

    #[test]
    fn evaluates_let_stmt() {
        let test_helper = AstTestHelper::<LetStmt>::new(Rule::let_stmt, "LetStmt");

        let environment = Environment::new();

        let result = test_helper
            .parse_and_evaluate(AssertEvaluateOptions {
                source: "let x = 42;",
                environment: Some(environment.clone()),
                context: None,
            })
            .expect("Failed to evaluate LetStmt");

        let RuntimeValue::None { .. } = result else {
            panic!("Expected RuntimeValue::None, got {:?}", result);
        };

        let RuntimeValue::Number { value, .. } =
            Environment::get_value(&environment, "x").expect("Variable x not found")
        else {
            panic!("Expected RuntimeValue::Number for variable x");
        };

        assert_eq!(value, 42.0);
    }

    #[test]
    fn evaluates_block_stmt() {
        let test_helper = AstTestHelper::<BlockStmt>::new(Rule::block_stmt, "BlockStmt");

        let environment = Environment::new();

        let result = test_helper
            .parse_and_evaluate(AssertEvaluateOptions {
                source: "{ let x = 10; let y = 20; }",
                environment: Some(environment.clone()),
                context: None,
            })
            .expect("Failed to evaluate BlockStmt");

        let block_environment = Environment::get_child(&environment)
            .expect("BlockStmt should create a child environment");

        let RuntimeValue::None { .. } = result else {
            panic!("Expected RuntimeValue::None, got {:?}", result);
        };

        let RuntimeValue::Number { value: x_value, .. } =
            Environment::get_value(&block_environment, "x").expect("Variable x not found")
        else {
            panic!("Expected RuntimeValue::Number for variable x");
        };

        assert_eq!(x_value, 10.0);

        let RuntimeValue::Number { value: y_value, .. } =
            Environment::get_value(&block_environment, "y").expect("Variable y not found")
        else {
            panic!("Expected RuntimeValue::Number for variable y");
        };

        assert_eq!(y_value, 20.0);
    }

    #[test]
    fn evaluates_expr_stmt() {
        let test_helper = AstTestHelper::<ExprStmt>::new(Rule::expression_stmt, "ExprStmt");

        let environment = Environment::new();

        Environment::insert(
            &environment,
            "x",
            RuntimeValue::Number {
                value: 30.0,
                span: OwnedSpan::default(),
            },
        );

        test_helper
            .parse_and_evaluate(AssertEvaluateOptions {
                source: "x = x + 50;",
                environment: Some(environment.clone()),
                context: None,
            })
            .expect("Failed to evaluate ExprStmt");

        let RuntimeValue::Number { value: x_value, .. } =
            Environment::get_value(&environment, "x").expect("Variable x not found")
        else {
            panic!("Expected RuntimeValue::Number for variable x");
        };

        assert_eq!(x_value, 80.0);
    }

    #[test]
    fn evaluates_if_stmt() {
        let test_helper = AstTestHelper::<BlockStmt>::new(Rule::block_stmt, "BlockStmt");

        let environment = Environment::new();

        let result = test_helper
            .parse_and_evaluate(AssertEvaluateOptions {
                source: r#"{
                    let x = 10;
                    let y = 5;
                    let foo = true;
                    if (foo) {
                        x = x + 100;
                    } else {
                        x = 200;
                    }
                    if(!!foo) {
                        y = y + 500;
                    }
                }"#,
                environment: Some(environment.clone()),
                context: None,
            })
            .expect("Failed to evaluate IfStmt");

        let block_environment = Environment::get_child(&environment)
            .expect("BlockStmt should create a child environment");

        let RuntimeValue::None { .. } = result else {
            panic!("Expected RuntimeValue::None, got {:?}", result);
        };

        let RuntimeValue::Number { value: x_value, .. } =
            Environment::get_value(&block_environment, "x").expect("Variable x not found")
        else {
            panic!("Expected RuntimeValue::Number for variable x");
        };

        assert_eq!(x_value, 110.0);

        let RuntimeValue::Number { value: y_value, .. } =
            Environment::get_value(&block_environment, "y").expect("Variable y not found")
        else {
            panic!("Expected RuntimeValue::Number for variable y");
        };
        assert_eq!(y_value, 505.0);
    }
}
