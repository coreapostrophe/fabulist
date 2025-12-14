//! Traits that make AST nodes evaluable in a runtime environment.
use crate::environment::RuntimeEnvironment;

pub mod runtime_value;

/// Contract for AST nodes that can be evaluated by the interpreter.
pub trait Evaluable {
    /// Concrete output type produced after evaluation.
    type Output;
    /// Evaluates the node using the given environment and story context.
    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
    ) -> Self::Output;
}
