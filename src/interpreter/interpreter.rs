use crate::expr::Expr;
use crate::token::Token;

pub fn visit_expr(mut expr: Expr) -> Token {
    expr.evaluate()
}
