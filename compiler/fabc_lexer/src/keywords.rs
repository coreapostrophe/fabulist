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

    // Iteration
    While,
    For,

    // Logical
    And,
    Or,
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
            _ => None,
        }
    }
}
