#[derive(Debug, Clone)]
pub enum ErrorKind {
    ExpectedToken { expected: String, found: String },
    UnrecognizedLiteral { literal: String },
    UnrecognizedPrimitive { primitive: String },
    UnrecognizedElement { element: String },
    UnrecognizedPrimary { primary: String },
    UnrecognizedInitiator { initiator: String },
    InvalidOperator { operator: String },
    UnclosedDelimiter,
}

impl ErrorKind {
    pub fn name(&self) -> &'static str {
        match self {
            ErrorKind::ExpectedToken { .. } => "Unexpected token",
            ErrorKind::UnrecognizedLiteral { .. } => "Unrecognized literal",
            ErrorKind::UnrecognizedPrimitive { .. } => "Unrecognized primitive",
            ErrorKind::UnrecognizedElement { .. } => "Unrecognized element",
            ErrorKind::UnrecognizedPrimary { .. } => "Unrecognized primary",
            ErrorKind::UnrecognizedInitiator { .. } => "Unrecognized initiator",
            ErrorKind::InvalidOperator { .. } => "Invalid operator",
            ErrorKind::UnclosedDelimiter => "Unclosed delimiter",
        }
    }
    pub fn message(&self) -> String {
        match self {
            ErrorKind::ExpectedToken { expected, found } => {
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
