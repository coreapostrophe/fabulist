#[derive(Debug, Clone, PartialEq)]
pub enum CompileErrorKind {
    ExpectedSymbol { expected: String, found: String },
    UnrecognizedLiteral { literal: String },
    UnrecognizedPrimitive { primitive: String },
    UnrecognizedElement { element: String },
    UnrecognizedPrimary { primary: String },
    UnrecognizedInitiator { initiator: String },
    InvalidOperator { operator: String },
    ExpectedType { expected: String, found: String },
    ArityMismatch { expected: usize, found: usize },
    InvalidMemberAccess { member: String },
    InvalidGotoTarget,
    TypeInference,
    UnclosedDelimiter,
    UninitializedVariable,
    NotCallable,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InternalErrorKind {
    InvalidAssignment,
    MissingIdentifierBinding { node_id: usize, name: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeErrorKind {
    InvalidLocalAddress,
    TypeMismatch,
    StackUnderflow,
    ConstantDoesNotExist,
    InstructionOutOfBounds,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    Compile(CompileErrorKind),
    Internal(InternalErrorKind),
    Runtime(RuntimeErrorKind),
}

impl From<CompileErrorKind> for ErrorKind {
    fn from(value: CompileErrorKind) -> Self {
        ErrorKind::Compile(value)
    }
}

impl From<InternalErrorKind> for ErrorKind {
    fn from(value: InternalErrorKind) -> Self {
        ErrorKind::Internal(value)
    }
}

impl From<RuntimeErrorKind> for ErrorKind {
    fn from(value: RuntimeErrorKind) -> Self {
        ErrorKind::Runtime(value)
    }
}

impl ErrorKind {
    pub fn name(&self) -> &'static str {
        match self {
            ErrorKind::Compile(kind) => kind.name(),
            ErrorKind::Internal(kind) => kind.name(),
            ErrorKind::Runtime(kind) => kind.name(),
        }
    }

    pub fn message(&self) -> String {
        match self {
            ErrorKind::Compile(kind) => kind.message(),
            ErrorKind::Internal(kind) => kind.message(),
            ErrorKind::Runtime(kind) => kind.message(),
        }
    }
}

impl CompileErrorKind {
    pub fn name(&self) -> &'static str {
        match self {
            CompileErrorKind::ExpectedType { .. } => "Unexpected type",
            CompileErrorKind::ExpectedSymbol { .. } => "Unexpected symbol",
            CompileErrorKind::UnrecognizedLiteral { .. } => "Unrecognized literal",
            CompileErrorKind::UnrecognizedPrimitive { .. } => "Unrecognized primitive",
            CompileErrorKind::UnrecognizedElement { .. } => "Unrecognized element",
            CompileErrorKind::UnrecognizedPrimary { .. } => "Unrecognized primary",
            CompileErrorKind::UnrecognizedInitiator { .. } => "Unrecognized initiator",
            CompileErrorKind::InvalidOperator { .. } => "Invalid operator",
            CompileErrorKind::ArityMismatch { .. } => "Arity mismatch",
            CompileErrorKind::InvalidMemberAccess { .. } => "Invalid member access",
            CompileErrorKind::InvalidGotoTarget => "Invalid goto target",
            CompileErrorKind::TypeInference => "Type Inference",
            CompileErrorKind::UnclosedDelimiter => "Unclosed delimiter",
            CompileErrorKind::UninitializedVariable => "Uninitialized variable",
            CompileErrorKind::NotCallable => "Not callable",
        }
    }

    pub fn message(&self) -> String {
        match self {
            CompileErrorKind::InvalidGotoTarget => {
                "The target of a goto statement must be a valid part identifier.".to_string()
            }
            CompileErrorKind::InvalidMemberAccess { member } => {
                format!("Invalid member access '{}'", member)
            }
            CompileErrorKind::ArityMismatch { expected, found } => {
                format!("Expected {} arguments, found {}", expected, found)
            }
            CompileErrorKind::NotCallable => "Attempted to call a non-callable entity".to_string(),
            CompileErrorKind::ExpectedType { expected, found } => {
                format!("Expected type '{}', found '{}'", expected, found)
            }
            CompileErrorKind::UninitializedVariable => {
                "Variable used before initialization".to_string()
            }
            CompileErrorKind::TypeInference => "Unable to infer type".to_string(),
            CompileErrorKind::ExpectedSymbol { expected, found } => {
                format!("Expected '{}', found '{}'", expected, found)
            }
            CompileErrorKind::UnrecognizedLiteral { literal } => {
                format!("Unrecognized literal '{}'", literal)
            }
            CompileErrorKind::UnrecognizedPrimitive { primitive } => {
                format!("Unrecognized primitive '{}'", primitive)
            }
            CompileErrorKind::UnrecognizedElement { element } => {
                format!("Unrecognized element '{}'", element)
            }
            CompileErrorKind::UnrecognizedPrimary { primary } => {
                format!("Unrecognized primary '{}'", primary)
            }
            CompileErrorKind::UnrecognizedInitiator { initiator } => {
                format!("Unrecognized initiator '{}'", initiator)
            }
            CompileErrorKind::InvalidOperator { operator } => {
                format!("Invalid operator '{}'", operator)
            }
            CompileErrorKind::UnclosedDelimiter => "Unclosed delimiter found".to_string(),
        }
    }
}

impl InternalErrorKind {
    pub fn name(&self) -> &'static str {
        match self {
            InternalErrorKind::InvalidAssignment => "Internal assignment error",
            InternalErrorKind::MissingIdentifierBinding { .. } => "Identifier binding missing",
        }
    }

    pub fn message(&self) -> String {
        match self {
            InternalErrorKind::InvalidAssignment => {
                "An internal error occurred during assignment".to_string()
            }
            InternalErrorKind::MissingIdentifierBinding { node_id, name } => {
                format!(
                    "Identifier '{}' is missing a symbol binding annotation (node id: {})",
                    name, node_id
                )
            }
        }
    }
}

impl RuntimeErrorKind {
    pub fn name(&self) -> &'static str {
        match self {
            RuntimeErrorKind::InvalidLocalAddress => "Invalid local address",
            RuntimeErrorKind::TypeMismatch => "Type mismatch",
            RuntimeErrorKind::StackUnderflow => "Stack underflow",
            RuntimeErrorKind::ConstantDoesNotExist => "Constant does not exist",
            RuntimeErrorKind::InstructionOutOfBounds => "Instruction pointer out of bounds",
        }
    }
    pub fn message(&self) -> String {
        match self {
            RuntimeErrorKind::InvalidLocalAddress => "Invalid local address".to_string(),
            RuntimeErrorKind::TypeMismatch => "Type mismatch".to_string(),
            RuntimeErrorKind::StackUnderflow => "Stack underflow".to_string(),
            RuntimeErrorKind::ConstantDoesNotExist => "Constant does not exist".to_string(),
            RuntimeErrorKind::InstructionOutOfBounds => {
                "Instruction pointer out of bounds".to_string()
            }
        }
    }
}
