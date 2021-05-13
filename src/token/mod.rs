#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,

    Comma,
    Period,
    Colon,

    // Single and double character tokens
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Star,
    StarEqual,
    Slash,
    SlashEqual,
    Percent,
    PercentEqual,
    Caret,
    CaretEqual,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(String),
    String(String),
    Number(f64),
    Bool(bool),

    // Keywords
    Func,
    Define,
    Return,
    Continue,
    Break,
    Print,

    And,
    Or,
    Not,

    If,
    Else,
    For,
    While,
    Loop,

    None,

    // Control tokens
    Newline,
    Indent,
    Dedent,

    Eof,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String) -> Token {
        Token { kind, lexeme }
    }

    pub fn to_string(&self) -> Option<String> {
        match self.kind.clone() {
            TokenKind::String(val) => Some(val),
            TokenKind::Number(val) => Some(val.to_string()),
            TokenKind::Bool(val) => Some(val.to_string()),

            TokenKind::None => Some(String::from("none")),

            _ => None,
        }
    }

    pub fn to_number(&self) -> Option<f64> {
        if let TokenKind::Number(val) = self.kind {
            Some(val)
        } else {
            None
        }
    }

    pub fn to_bool(&self) -> Option<bool> {
        match self.kind.clone() {
            TokenKind::Identifier(_val) => Some(false), // TODO: Return true if an identifier exists within the current scope, else return false
            TokenKind::String(val) => Some(!val.is_empty()),
            TokenKind::Number(val) => {
                if val != 0.0 {
                    Some(true)
                } else {
                    Some(false)
                }
            }

            TokenKind::Bool(val) => Some(val),
            TokenKind::None => Some(false),

            _ => None,
        }
    }

    pub fn is_equal(&self, rhs: Token) -> bool {
        // TODO: Make less ugly
        if self.to_number().is_some() && rhs.to_number().is_some() {
            self.to_number().unwrap() == rhs.to_number().unwrap()
        } else if self.to_bool().is_some() && rhs.to_bool().is_some() {
            self.to_bool().unwrap() == rhs.to_bool().unwrap()
        } else if self.to_string().is_some() && self.to_string().is_some() {
            self.to_string().unwrap() == rhs.to_string().unwrap()
        } else {
            false
        }
    }
}

impl From<TokenKind> for Token {
    fn from(kind: TokenKind) -> Token {
        Token {
            kind,
            lexeme: String::new(),
        }
    }
}
