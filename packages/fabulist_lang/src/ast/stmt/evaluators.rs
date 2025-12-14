use crate::{
    ast::stmt::models::{BlockStmt, ElseClause, GotoStmt, IfStmt, LetStmt, Stmt},
    environment::{Environment, RuntimeEnvironment},
    error::RuntimeError,
    interpreter::{runtime_value::RuntimeValue, Evaluable},
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
        let condition = self.condition.evaluate(environment, context)?;

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
        }
    }
}
