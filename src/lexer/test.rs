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
            Token::from(TokenKind::LeftBracket),
            Token::from(TokenKind::PlusEqual),
            Token::from(TokenKind::RightBracket),
            Token::from(TokenKind::Newline),
        ]
    );
}

#[test]
fn lex_keyword() {
    assert_eq!(
        lex("if"),
        vec![
            Token::new(TokenKind::If, String::from("if")),
            Token::from(TokenKind::Newline),
        ]
    );
}

#[test]
fn lex_identifier() {
    assert_eq!(
        lex("foo"),
        vec![
            Token::new(TokenKind::Identifier(String::from("foo")), String::from("foo")),
            Token::from(TokenKind::Newline),
        ]
    );
}

#[test]
fn lex_number() {
    assert_eq!(
        lex("3.14"),
        vec![
            Token::new(TokenKind::Number(3.14), String::from("3.14")),
            Token::from(TokenKind::Newline)
        ]
    );
}

#[test]
fn lex_string() {
    assert_eq!(
        lex("'Hello' \"world\""),
        vec![
            Token::new(TokenKind::String(String::from("Hello")), String::from("Hello")),
            Token::new(TokenKind::String(String::from("world")), String::from("world")),
            Token::from(TokenKind::Newline)
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
            Token::new(TokenKind::If, String::from("if")),
            Token::from(TokenKind::Newline),
            Token::from(TokenKind::Indent),
            Token::new(TokenKind::Identifier(String::from("then")), String::from("then")),
            Token::from(TokenKind::Newline),
            Token::from(TokenKind::Dedent),
        ]
    );
}
