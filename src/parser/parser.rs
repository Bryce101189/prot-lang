use crate::{expr::Expr, statement::Statement};
use crate::token::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,

    pub contains_errors: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            cursor: 0,

            contains_errors: false,
        }
    }

    pub fn reset(&mut self) {
        self.cursor = 0;
        self.contains_errors = false;
    }

    fn reached_end(&self) -> bool {
        self.cursor >= self.tokens.len()
    }

    fn peek(&self) -> Token {
        self.tokens[self.cursor].clone()
    }

    fn advance(&mut self) -> Token {
        let t = self.peek();
        self.cursor += 1;
        t
    }

    fn is_match(&self, kind: TokenKind) -> bool {
        if self.reached_end() {
            false
        } else {
            kind == self.peek().kind
        }
    }

    fn expect(&mut self, kind: TokenKind) {
        let tokens = &self.tokens[self.cursor..];

        for token in tokens {
            if token.kind == kind {
                return;
            }
        }

        eprintln!("Parsing error: expected token of type {:?}.\n", kind);
        self.contains_errors = true;
    }

    fn parse_primary(&mut self) -> Expr {
        match self.peek().kind {
            // Literals
            TokenKind::None
            | TokenKind::Bool(..)
            | TokenKind::Number(..)
            | TokenKind::String(..)
            | TokenKind::Identifier(..) => Expr::Literal(self.advance()),

            // Groupings
            TokenKind::LeftParen => {
                let lhs = self.advance();
                let expr = self.parse_expression();
                self.expect(TokenKind::RightParen);

                Expr::Grouping(lhs, Box::new(expr), Token::from(TokenKind::RightParen))
            }

            TokenKind::LeftBracket => {
                let lhs = self.advance();
                let expr = self.parse_expression();
                self.expect(TokenKind::RightBracket);

                Expr::Grouping(lhs, Box::new(expr), Token::from(TokenKind::RightBracket))
            }

            TokenKind::Indent => {
                let lhs = self.advance();
                let expr = self.parse_expression();
                self.expect(TokenKind::Dedent);

                Expr::Grouping(lhs, Box::new(expr), Token::from(TokenKind::Dedent))
            }

            _ => Expr::Literal(Token::from(TokenKind::Eof)),
        }
    }

    fn parse_unary(&mut self) -> Expr {
        while self.is_match(TokenKind::Bang)
            || self.is_match(TokenKind::Not)
            || self.is_match(TokenKind::Minus)
        {
            let op = self.advance();
            let rhs = self.parse_unary();

            return Expr::Unary(op, Box::new(rhs));
        }

        self.parse_primary()
    }

    fn parse_exponent(&mut self) -> Expr {
        let mut expr = self.parse_unary();

        while self.is_match(TokenKind::Caret) {
            let op = self.advance();
            let rhs = self.parse_unary();
            expr = Expr::Binary(Box::new(expr), op, Box::new(rhs));
        }

        expr
    }

    fn parse_factor(&mut self) -> Expr {
        let mut expr = self.parse_exponent();

        while self.is_match(TokenKind::Star)
            || self.is_match(TokenKind::Slash)
            || self.is_match(TokenKind::Percent)
        {
            let op = self.advance();
            let rhs = self.parse_exponent();
            expr = Expr::Binary(Box::new(expr), op, Box::new(rhs));
        }

        expr
    }

    fn parse_term(&mut self) -> Expr {
        let mut expr = self.parse_factor();

        while self.is_match(TokenKind::Plus) || self.is_match(TokenKind::Minus) {
            let op = self.advance();
            let rhs = self.parse_factor();
            expr = Expr::Binary(Box::new(expr), op, Box::new(rhs));
        }

        expr
    }

    fn parse_comparison(&mut self) -> Expr {
        let mut expr = self.parse_term();

        while self.is_match(TokenKind::Greater)
            || self.is_match(TokenKind::GreaterEqual)
            || self.is_match(TokenKind::Less)
            || self.is_match(TokenKind::LessEqual)
        {
            let op = self.advance();
            let rhs = self.parse_term();
            expr = Expr::Binary(Box::new(expr), op, Box::new(rhs));
        }

        expr
    }

    fn parse_equality(&mut self) -> Expr {
        let mut expr = self.parse_comparison();

        while self.is_match(TokenKind::EqualEqual) || self.is_match(TokenKind::BangEqual) {
            let op = self.advance();
            let rhs = self.parse_comparison();
            expr = Expr::Binary(Box::new(expr), op, Box::new(rhs));
        }

        expr
    }

    fn parse_expression(&mut self) -> Expr {
        self.parse_equality()
    }

    fn parse_print(&mut self) -> Statement {
        let expr = self.parse_expression();
        self.expect(TokenKind::Newline);
        Statement::Print(expr)
    }

    fn parse_statement(&mut self) -> Statement {
        match self.advance().kind {
            TokenKind::Print => self.parse_print(),

            _ => Statement::Expr(self.parse_expression()),
        }
    }

    pub fn parse_tokens(&mut self) -> Vec<Statement> {
        self.reset();
        let mut statements = Vec::new();

        while self.cursor < self.tokens.len() - 1 {
            let statement = self.parse_statement();
            statements.push(statement);
        }

        statements
    }
}
