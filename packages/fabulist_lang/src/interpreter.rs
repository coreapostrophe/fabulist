use crate::environment::RuntimeEnvironment;

pub mod runtime_value;

pub trait Evaluable {
    type Output;
    fn evaluate(
        &self,
        environment: &RuntimeEnvironment,
        context: &RuntimeEnvironment,
    ) -> Self::Output;
}
