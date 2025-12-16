//! Runtime value representation used by the interpreter.
use std::collections::HashMap;

use crate::{
    error::RuntimeError,
    interpreter::environment::RuntimeEnvironment,
    parser::{
        ast::{dfn::models::ParameterBodyDfn, stmt::models::BlockStmt},
        error::SpanSlice,
    },
};

pub mod overrides;

/// Values produced and consumed by the interpreter.
#[derive(Clone, Debug)]
pub enum RuntimeValue {
    /// Floating-point number literal.
    Number {
        /// Numeric payload.
        value: f32,
        /// Source span of the literal.
        span: SpanSlice,
    },
    /// Boolean literal.
    Boolean {
        /// Boolean payload.
        value: bool,
        /// Source span of the literal.
        span: SpanSlice,
    },
    /// UTF-8 string literal.
    String {
        /// String payload.
        value: String,
        /// Source span of the literal.
        span: SpanSlice,
    },
    /// Object literal with string keys and runtime values.
    Object {
        /// Object properties keyed by string.
        properties: HashMap<String, RuntimeValue>,
        /// Source span of the primitive.
        span: SpanSlice,
    },
    /// Lambda defined in source code with captured closure.
    Lambda {
        /// Parameters captured from the lambda head.
        parameters: ParameterBodyDfn,
        /// Lambda body.
        body: BlockStmt,
        /// Captured runtime environment.
        closure: RuntimeEnvironment,
        /// Source span of the primitive.
        span: SpanSlice,
    },
    /// Native (Rust) function exposed to the runtime.
    NativeFunction(fn(Vec<RuntimeValue>, SpanSlice) -> Result<RuntimeValue, RuntimeError>),
    /// Identifier pending resolution in an environment.
    Identifier {
        /// Identifier text.
        name: String,
        /// Source span of the identifier.
        span: SpanSlice,
    },
    /// Marker for the absence of a value.
    None {
        /// Source span of the literal.
        span: SpanSlice,
    },
    /// Handle to the mutable story context.
    Context {
        /// Span referencing the `context` primitive.
        span: SpanSlice,
    },
    /// Path literal.
    Path {
        /// Path segments.
        segments: Vec<String>,
        /// Source span of the primitive.
        span: SpanSlice,
    },
    /// Module handle with its own environment.
    Module {
        /// Module environment.
        environment: RuntimeEnvironment,
        /// Source span of the primitive.
        span: SpanSlice,
    },
}

impl RuntimeValue {
    /// Human-readable name for the variant, useful for diagnostics.
    pub fn type_name(&self) -> String {
        match self {
            RuntimeValue::Number { .. } => "Number".to_string(),
            RuntimeValue::Boolean { .. } => "Boolean".to_string(),
            RuntimeValue::String { .. } => "String".to_string(),
            RuntimeValue::Identifier { .. } => "Identifier".to_string(),
            RuntimeValue::Object { .. } => "Object".to_string(),
            RuntimeValue::Lambda { .. } => "Lambda".to_string(),
            RuntimeValue::NativeFunction(_) => "NativeFunction".to_string(),
            RuntimeValue::None { .. } => "None".to_string(),
            RuntimeValue::Context { .. } => "Context".to_string(),
            RuntimeValue::Path { .. } => "Path".to_string(),
            RuntimeValue::Module { .. } => "Module".to_string(),
        }
    }
}
