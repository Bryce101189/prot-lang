use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};

fn lex(source: &str) -> Vec<Token> {
    Lexer::new(String::from(source)).collect_tokens()
}

#[test]
fn lex_nothing() {
    assert_eq!(lex(""), vec![]);
}

#[test]
fn lex_whitespace() {
    assert_eq!(lex("    "), vec![]);
}

#[test]
fn lex_comment() {
    assert_eq!(lex("# this is a comment"), vec![]);
}

#[test]
fn lex_symbols() {
    assert_eq!(
        lex("[ += ]"),
        vec![
            Token {
                kind: TokenKind::LeftBracket,
                lexeme: String::new()
            },
            Token {
                kind: TokenKind::PlusEqual,
                lexeme: String::new()
            },
            Token {
                kind: TokenKind::RightBracket,
                lexeme: String::new()
            },
            Token {
                kind: TokenKind::Newline,
                lexeme: String::new()
            }
        ]
    );
}

#[test]
fn lex_keyword() {
    assert_eq!(
        lex("if"),
        vec![
            Token {
                kind: TokenKind::If,
                lexeme: String::from("if")
            },
            Token {
                kind: TokenKind::Newline,
                lexeme: String::new()
            }
        ]
    );
}

#[test]
fn lex_identifier() {
    assert_eq!(
        lex("foo"),
        vec![
            Token {
                kind: TokenKind::Identifier(String::from("foo")),
                lexeme: String::from("foo")
            },
            Token {
                kind: TokenKind::Newline,
                lexeme: String::new()
            }
        ]
    );
}

#[test]
fn lex_number() {
    assert_eq!(
        lex("3.14"),
        vec![
            Token {
                kind: TokenKind::Number(3.14),
                lexeme: String::from("3.14")
            },
            Token {
                kind: TokenKind::Newline,
                lexeme: String::new()
            }
        ]
    );
}

#[test]
fn lex_string() {
    assert_eq!(
        lex("'Hello' \"world\""),
        vec![
            Token {
                kind: TokenKind::String(String::from("Hello")),
                lexeme: String::from("Hello")
            },
            Token {
                kind: TokenKind::String(String::from("world")),
                lexeme: String::from("world")
            },
            Token {
                kind: TokenKind::Newline,
                lexeme: String::new()
            }
        ]
    );
}

#[test]
fn lex_indentation() {
    assert_eq!(
        lex(r#"
if
    then
        "#),
        vec![
            Token {
                kind: TokenKind::If,
                lexeme: String::from("if")
            },
            Token {
                kind: TokenKind::Newline,
                lexeme: String::new()
            },
            Token {
                kind: TokenKind::Indent,
                lexeme: String::new()
            },
            Token {
                kind: TokenKind::Identifier(String::from("then")),
                lexeme: String::from("then")
            },
            Token {
                kind: TokenKind::Newline,
                lexeme: String::new()
            },
            Token {
                kind: TokenKind::Dedent,
                lexeme: String::new()
            },
        ]
    );
}
