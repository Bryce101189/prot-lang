use crate::token::{Token, TokenKind};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Literal(Token),
    Grouping(Token, Box<Expr>, Token),
}

impl Expr {
    pub fn evaluate(&mut self) -> Token {
        match self {
            Expr::Unary(..) => self.evaluate_unary(),
            Expr::Binary(..) => self.evaluate_binary(),
            Expr::Literal(..) => self.evaluate_literal(),
            Expr::Grouping(..) => self.evaluate_grouping(),
        }
    }

    fn evaluate_unary(&mut self) -> Token {
        match self {
            Expr::Unary(op, expr) => {
                let rhs = expr.evaluate();

                let kind = match op.kind {
                    TokenKind::Minus => {
                        let val = rhs.to_number();

                        if let None = val {
                            eprintln!(
                                "Conversion error: Could not convert type {:?} to Number.\n",
                                rhs.kind,
                            );

                            TokenKind::None
                        } else {
                            TokenKind::Number(-val.unwrap())
                        }
                    }

                    TokenKind::Bang | TokenKind::Not => {
                        let val = rhs.to_bool();

                        if let None = val {
                            eprintln!(
                                "Conversion error: Could not convert type {:?} to Bool.\n",
                                rhs.kind,
                            );

                            TokenKind::None
                        } else {
                            TokenKind::Bool(!val.unwrap())
                        }
                    }

                    _ => {
                        eprintln!(
                            "Expression error: Could not apply operation {:?} to expression {:?}.\n",
                            op.kind, expr
                        );

                        TokenKind::None
                    }
                };

                Token::from(kind)
            }

            _ => Token::from(TokenKind::None),
        }
    }

    fn evaluate_binary(&mut self) -> Token {
        match self {
            Expr::Binary(lhs, op, rhs) => {
                let lhs = lhs.evaluate();
                let rhs = rhs.evaluate();

                let kind = match op.kind {
                    TokenKind::Plus => match (lhs.clone().kind, rhs.clone().kind) {
                        (TokenKind::Number(..), TokenKind::Number(..)) => {
                            TokenKind::Number(lhs.to_number().unwrap() + rhs.to_number().unwrap())
                        }

                        (TokenKind::String(..), TokenKind::String(..)) => TokenKind::String(
                            lhs.to_string().unwrap() + rhs.to_string().unwrap().as_str(),
                        ),

                        _ => {
                            eprintln!(
                                    "Expression error: Could not apply operation {:?} to expressions {:?} and {:?}.\n",
                                    op.kind, lhs.kind, rhs.kind 
                                );

                            TokenKind::None
                        }
                    },
                    TokenKind::Minus => {
                        let lval = lhs.to_number();
                        let rval = rhs.to_number();

                        match (lval, rval) {
                            (Some(lval), Some(rval)) => TokenKind::Number(lval - rval),

                            _ => {
                                eprintln!(
                                    "Expression error: Could not apply operation {:?} to expressions {:?} and {:?}.\n",
                                    op.kind, lhs.kind, rhs.kind 
                                );

                                TokenKind::None
                            }
                        }
                    }
                    TokenKind::Star => {
                        let lval = lhs.to_number();
                        let rval = rhs.to_number();

                        match (lval, rval) {
                            (Some(lval), Some(rval)) => TokenKind::Number(lval * rval),

                            _ => {
                                eprintln!(
                                    "Expression error: Could not apply operation {:?} to expressions {:?} and {:?}.\n",
                                    op.kind, lhs.kind, rhs.kind
                                );

                                TokenKind::None
                            }
                        }
                    }
                    TokenKind::Slash => {
                        let lval = lhs.to_number();
                        let rval = rhs.to_number();

                        match (lval, rval) {
                            (Some(lval), Some(rval)) => TokenKind::Number(lval / rval),

                            _ => {
                                eprintln!(
                                    "Expression error: Could not apply operation {:?} to expressions {:?} and {:?}.\n",
                                    op.kind, lhs.kind, rhs.kind 
                                );

                                TokenKind::None
                            }
                        }
                    }
                    TokenKind::Percent => {
                        let lval = lhs.to_number();
                        let rval = rhs.to_number();

                        match (lval, rval) {
                            (Some(lval), Some(rval)) => TokenKind::Number(lval % rval),

                            _ => {
                                eprintln!(
                                    "Expression error: Could not apply operation {:?} to expressions {:?} and {:?}.\n",
                                    op.kind, lhs.kind, rhs.kind 
                                );

                                TokenKind::None
                            }
                        }
                    }
                    TokenKind::Caret => {
                        let lval = lhs.to_number();
                        let rval = rhs.to_number();

                        match (lval, rval) {
                            (Some(lval), Some(rval)) => TokenKind::Number(lval.powf(rval)),

                            _ => {
                                eprintln!(
                                    "Expression error: Could not apply operation {:?} to expressions {:?} and {:?}.\n",
                                    op.kind, lhs.kind, rhs.kind 
                                );

                                TokenKind::None
                            }
                        }
                    }

                    TokenKind::Greater => {
                        let lval = lhs.to_number();
                        let rval = rhs.to_number();

                        match (lval, rval) {
                            (Some(lval), Some(rval)) => TokenKind::Bool(lval > rval),

                            _ => {
                                eprintln!(
                                    "Expression error: Could not apply operation {:?} to expressions {:?} and {:?}.\n",
                                    op.kind, lhs.kind, rhs.kind 
                                );

                                TokenKind::None
                            }
                        }
                    }
                    TokenKind::GreaterEqual => {
                        let lval = lhs.to_number();
                        let rval = rhs.to_number();

                        match (lval, rval) {
                            (Some(lval), Some(rval)) => TokenKind::Bool(lval >= rval),

                            _ => {
                                eprintln!(
                                    "Expression error: Could not apply operation {:?} to expressions {:?} and {:?}.\n",
                                    op.kind, lhs.kind, rhs.kind 
                                );

                                TokenKind::None
                            }
                        }
                    }
                    TokenKind::Less => {
                        let lval = lhs.to_number();
                        let rval = rhs.to_number();

                        match (lval, rval) {
                            (Some(lval), Some(rval)) => TokenKind::Bool(lval < rval),

                            _ => {
                                eprintln!(
                                    "Expression error: Could not apply operation {:?} to expressions {:?} and {:?}.\n",
                                    op.kind, lhs.kind, rhs.kind 
                                );

                                TokenKind::None
                            }
                        }
                    }
                    TokenKind::LessEqual => {
                        let lval = lhs.to_number();
                        let rval = rhs.to_number();

                        match (lval, rval) {
                            (Some(lval), Some(rval)) => TokenKind::Bool(lval <= rval),

                            _ => {
                                eprintln!(
                                    "Expression error: Could not apply operation {:?} to expressions {:?} and {:?}.\n",
                                    op.kind, lhs.kind, rhs.kind 
                                );

                                TokenKind::None
                            }
                        }
                    }

                    TokenKind::EqualEqual => TokenKind::Bool(lhs.is_equal(rhs)),
                    TokenKind::BangEqual => TokenKind::Bool(!lhs.is_equal(rhs)),

                    _ => TokenKind::None,
                };

                Token::from(kind)
            }

            _ => Token::from(TokenKind::None),
        }
    }

    fn evaluate_literal(&mut self) -> Token {
        match self {
            Expr::Literal(tok) => tok.clone(),

            _ => Token::from(TokenKind::None),
        }
    }

    fn evaluate_grouping(&mut self) -> Token {
        Token::from(TokenKind::None)
    }
}
