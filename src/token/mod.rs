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

    True,
    False,
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
}

impl From<TokenKind> for Token {
    fn from(kind: TokenKind) -> Token {
        Token {
            kind,
            lexeme: String::new(),
        }
    }
}
