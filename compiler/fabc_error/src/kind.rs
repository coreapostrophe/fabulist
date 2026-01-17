#[derive(Debug, Clone)]
pub enum ErrorKind {
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
    TypeInference,
    InternalAssignment,
    UnclosedDelimiter,
    UninitializedVariable,
    NotCallable,
}

impl ErrorKind {
    pub fn name(&self) -> &'static str {
        match self {
            ErrorKind::ExpectedType { .. } => "Unexpected type",
            ErrorKind::ExpectedSymbol { .. } => "Unexpected symbol",
            ErrorKind::UnrecognizedLiteral { .. } => "Unrecognized literal",
            ErrorKind::UnrecognizedPrimitive { .. } => "Unrecognized primitive",
            ErrorKind::UnrecognizedElement { .. } => "Unrecognized element",
            ErrorKind::UnrecognizedPrimary { .. } => "Unrecognized primary",
            ErrorKind::UnrecognizedInitiator { .. } => "Unrecognized initiator",
            ErrorKind::InvalidOperator { .. } => "Invalid operator",
            ErrorKind::ArityMismatch { .. } => "Arity mismatch",
            ErrorKind::InvalidMemberAccess { .. } => "Invalid member access",
            ErrorKind::TypeInference => "Type Inference",
            ErrorKind::InternalAssignment => "Internal assignment error",
            ErrorKind::UnclosedDelimiter => "Unclosed delimiter",
            ErrorKind::UninitializedVariable => "Uninitialized variable",
            ErrorKind::NotCallable => "Not callable",
        }
    }
    pub fn message(&self) -> String {
        match self {
            ErrorKind::InvalidMemberAccess { member } => {
                format!("Invalid member access '{}'", member)
            }
            ErrorKind::ArityMismatch { expected, found } => {
                format!("Expected {} arguments, found {}", expected, found)
            }
            ErrorKind::NotCallable => "Attempted to call a non-callable entity".to_string(),
            ErrorKind::ExpectedType { expected, found } => {
                format!("Expected type '{}', found '{}'", expected, found)
            }
            ErrorKind::UninitializedVariable => "Variable used before initialization".to_string(),
            ErrorKind::InternalAssignment => {
                "An internal error occurred during assignment".to_string()
            }
            ErrorKind::TypeInference => "Unable to infer type".to_string(),
            ErrorKind::ExpectedSymbol { expected, found } => {
                format!("Expected '{}', found '{}'", expected, found)
            }
            ErrorKind::UnrecognizedLiteral { literal } => {
                format!("Unrecognized literal '{}'", literal)
            }
            ErrorKind::UnrecognizedPrimitive { primitive } => {
                format!("Unrecognized primitive '{}'", primitive)
            }
            ErrorKind::UnrecognizedElement { element } => {
                format!("Unrecognized element '{}'", element)
            }
            ErrorKind::UnrecognizedPrimary { primary } => {
                format!("Unrecognized primary '{}'", primary)
            }
            ErrorKind::UnrecognizedInitiator { initiator } => {
                format!("Unrecognized initiator '{}'", initiator)
            }
            ErrorKind::InvalidOperator { operator } => {
                format!("Invalid operator '{}'", operator)
            }
            ErrorKind::UnclosedDelimiter => "Unclosed delimiter found".to_string(),
        }
    }
}
