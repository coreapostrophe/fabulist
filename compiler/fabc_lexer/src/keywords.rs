use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum KeywordKind {
    // Declarations
    Let,
    Fn,

    // Control Flow
    If,
    Else,
    Return,
    Goto,

    // Literals
    True,
    False,
    None,

    // Primitive
    Context,

    // Iteration
    While,
    For,

    // Logical
    And,
    Or,
}

impl Display for KeywordKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeywordKind::Let => write!(f, "let"),
            KeywordKind::Fn => write!(f, "fn"),
            KeywordKind::If => write!(f, "if"),
            KeywordKind::Else => write!(f, "else"),
            KeywordKind::Return => write!(f, "return"),
            KeywordKind::Goto => write!(f, "goto"),
            KeywordKind::True => write!(f, "true"),
            KeywordKind::False => write!(f, "false"),
            KeywordKind::None => write!(f, "none"),
            KeywordKind::While => write!(f, "while"),
            KeywordKind::For => write!(f, "for"),
            KeywordKind::And => write!(f, "and"),
            KeywordKind::Or => write!(f, "or"),
            KeywordKind::Context => write!(f, "context"),
        }
    }
}

impl KeywordKind {
    pub fn get(s: &str) -> Option<KeywordKind> {
        match s {
            "let" => Some(KeywordKind::Let),
            "fn" => Some(KeywordKind::Fn),
            "if" => Some(KeywordKind::If),
            "else" => Some(KeywordKind::Else),
            "return" => Some(KeywordKind::Return),
            "goto" => Some(KeywordKind::Goto),
            "true" => Some(KeywordKind::True),
            "false" => Some(KeywordKind::False),
            "none" => Some(KeywordKind::None),
            "while" => Some(KeywordKind::While),
            "for" => Some(KeywordKind::For),
            "and" => Some(KeywordKind::And),
            "or" => Some(KeywordKind::Or),
            "context" => Some(KeywordKind::Context),
            _ => None,
        }
    }
}
