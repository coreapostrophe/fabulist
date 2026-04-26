use std::fmt::Debug;

use fabc_ir::FunctionId;

use super::{ObjectRef, Scope, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct CompiledInvocationResult {
    pub value: Value,
    pub goto: Option<String>,
}

pub trait CompiledFunctionHost: Debug {
    fn invoke_function(
        &self,
        function_id: FunctionId,
        captured: Scope,
        context: ObjectRef,
        args: Vec<Value>,
    ) -> Result<CompiledInvocationResult, String>;

    fn resolve_function_symbol(&self, symbol: &str) -> Result<FunctionId, String> {
        Err(format!(
            "compiled function host cannot resolve symbol `{symbol}`"
        ))
    }
}
