use fabc_ir::FunctionId;

use super::{ObjectRef, Scope, Value};

#[derive(Debug, Clone, PartialEq)]
pub struct CompiledInvocationResult {
    pub value: Value,
    pub goto: Option<String>,
}

pub trait CompiledFunctionHost: std::fmt::Debug {
    fn invoke_function(
        &self,
        function_id: FunctionId,
        captured: Scope,
        context: ObjectRef,
        args: Vec<Value>,
    ) -> std::result::Result<CompiledInvocationResult, String>;
}
