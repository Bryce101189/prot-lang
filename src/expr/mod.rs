use crate::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Literal(Token),
    Grouping(Token, Box<Expr>, Token),
}
