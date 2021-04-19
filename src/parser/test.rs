use crate::token::{Token, TokenKind};
use crate::expr::Expr;
use crate::parser::Parser;

fn parse(tokens: Vec<Token>) -> Expr {
    Parser::new(tokens).parse_tokens()
}

#[test]
fn parse_equality() {
    assert_eq!(
        parse(vec![
            Token::from(TokenKind::Number(8.7)),
            Token::from(TokenKind::EqualEqual),
            Token::from(TokenKind::Number(3.14)),
        ]),

        Expr::Binary(
            Box::new(
                Expr::Literal(
                    Token::from(TokenKind::Number(8.7))
                )
            ),
            Token::from(TokenKind::EqualEqual),
            Box::new(
                Expr::Literal(
                    Token::from(TokenKind::Number(3.14))
                )
            ),
        )
    );
}

#[test]
fn parse_comparison() {
    assert_eq!(
        parse(vec![
            Token::from(TokenKind::Number(8.7)),
            Token::from(TokenKind::GreaterEqual),
            Token::from(TokenKind::Number(3.14)),
        ]),

        Expr::Binary(
            Box::new(
                Expr::Literal(
                    Token::from(TokenKind::Number(8.7))
                )
            ),
            Token::from(TokenKind::GreaterEqual),
            Box::new(
                Expr::Literal(
                    Token::from(TokenKind::Number(3.14))
                )
            ),
        )
    );
}

#[test]
fn parse_term() {
    assert_eq!(
        parse(vec![
            Token::from(TokenKind::Number(8.7)),
            Token::from(TokenKind::Plus),
            Token::from(TokenKind::Number(3.14)),
        ]),

        Expr::Binary(
            Box::new(
                Expr::Literal(
                    Token::from(TokenKind::Number(8.7))
                )
            ),
            Token::from(TokenKind::Plus),
            Box::new(
                Expr::Literal(
                    Token::from(TokenKind::Number(3.14))
                )
            ),
        )
    );
}

#[test]
fn parse_factor() {
    assert_eq!(
        parse(vec![
            Token::from(TokenKind::Number(8.7)),
            Token::from(TokenKind::Star),
            Token::from(TokenKind::Number(3.14)),
        ]),

        Expr::Binary(
            Box::new(
                Expr::Literal(
                    Token::from(TokenKind::Number(8.7))
                )
            ),
            Token::from(TokenKind::Star),
            Box::new(
                Expr::Literal(
                    Token::from(TokenKind::Number(3.14))
                )
            ),
        )
    );
}

#[test]
fn parse_exponent() {
    assert_eq!(
        parse(vec![
            Token::from(TokenKind::Number(8.7)),
            Token::from(TokenKind::Caret),
            Token::from(TokenKind::Number(3.14)),
        ]),

        Expr::Binary(
            Box::new(
                Expr::Literal(
                    Token::from(TokenKind::Number(8.7))
                )
            ),
            Token::from(TokenKind::Caret),
            Box::new(
                Expr::Literal(
                    Token::from(TokenKind::Number(3.14))
                )
            ),
        )
    );
}

#[test]
fn parse_unary() {
    assert_eq!(
        parse(vec![
            Token::from(TokenKind::Minus),
            Token::from(TokenKind::Number(3.14)),
        ]),

        Expr::Unary(
            Token::from(TokenKind::Minus),
            Box::new(
                Expr::Literal(
                    Token::from(TokenKind::Number(3.14))
                )
            ),
        )
    );
}

#[test]
fn parse_literal() {
    assert_eq!(
        parse(vec![
            Token::from(TokenKind::Number(8.7)),
        ]),

        Expr::Literal(
            Token::from(TokenKind::Number(8.7))
        )
    );
}

#[test]
fn parse_grouping() {
    assert_eq!(
        parse(vec![
            Token::from(TokenKind::LeftBracket),
            Token::new(TokenKind::Identifier(String::from("a")), String::from("a")),
            Token::from(TokenKind::Plus),
            Token::from(TokenKind::Number(5.0)),
            Token::from(TokenKind::RightBracket),
        ]),

        Expr::Grouping(
            Token::from(TokenKind::LeftBracket),
            Box::new(
                Expr::Binary(
                    Box::new(
                        Expr::Literal(
                            Token::new(TokenKind::Identifier(String::from("a")), String::from("a"))
                        )
                    ),

                    Token::from(TokenKind::Plus),
                    
                    Box::new(
                        Expr::Literal(
                            Token::from(TokenKind::Number(5.0))
                        )
                    )
                )
            ),
            Token::from(TokenKind::RightBracket),
        )
    );
}
