use crate::token::{Token, TokenKind};

pub struct Lexer {
    source: Vec<char>,
    cursor: usize,

    tokens: Vec<Token>,

    location: (usize, usize),
    token_location: (usize, usize),

    at_new_line: bool,
    pub contains_errors: bool,

    indent_stack: Vec<usize>,
    indent_counter: usize,
    bracket_counter: usize,
    paren_counter: usize,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source: source.chars().collect(),
            cursor: 0,

            tokens: Vec::new(),

            location: (0, 0),
            token_location: (0, 0),

            at_new_line: true,
            contains_errors: false,

            indent_stack: vec![0],
            indent_counter: 0,
            bracket_counter: 0,
            paren_counter: 0,
        }
    }

    pub fn reset(&mut self) {
        self.tokens.clear();
        self.cursor = 0;

        self.location = (0, 0);
        self.token_location = (0, 0);

        self.at_new_line = true;
        self.contains_errors = false;

        self.indent_stack = vec![0];
        self.indent_counter = 0;
        self.bracket_counter = 0;
        self.paren_counter = 0;
    }

    fn reached_end(&self) -> bool {
        self.cursor >= self.source.len()
    }

    fn peek(&self) -> char {
        if self.reached_end() {
            return '\0';
        }

        self.source[self.cursor]
    }

    fn advance(&mut self) -> char {
        let c = self.peek();

        self.cursor += 1;
        self.location.1 += 1;
        self.at_new_line = false;

        c
    }

    fn consume_match(&mut self, case: char) -> bool {
        if case == self.peek() {
            self.advance();
            true
        } else {
            false
        }
    }

    fn at_empty_line(&self) -> bool {
        let mut cursor = self.cursor;

        while cursor < self.source.len() {
            let c = self.source[cursor];
            cursor += 1;

            if c == '\n' || c == '#' {
                return true;
            } else if !c.is_whitespace() {
                return false;
            }
        }

        true
    }

    fn skip_whitespace(&mut self) {
        // Newline characters are not skipped as they are meaningful characters
        while !self.reached_end() && self.peek().is_whitespace() && self.peek() != '\n' {
            self.advance();
        }
    }

    fn skip_line(&mut self) {
        while !self.reached_end() && self.peek() != '\n' {
            self.advance();
        }
    }

    fn get_indentation_level(&mut self) {
        let mut spaces = 0;

        while !self.reached_end() && (self.peek() == ' ' || self.peek() == '\t') {
            if self.peek() == ' ' {
                spaces += 1;
            } else if self.peek() == '\t' {
                spaces += 4; // Very hacky fix for allowing the use of tabs...
            }

            self.advance();
        }

        // Return if the indentation level hasn't changed
        if spaces == *self.indent_stack.last().unwrap() {
            self.indent_stack.push(spaces);
            return;
        } else if spaces > *self.indent_stack.last().unwrap() {
            self.indent_counter += 1;
            self.tokens.push(Token::from(TokenKind::Indent));
        }

        // Go down the indentation stack, keeping track of the lowest indentation level seen that is still greater than
        // the current line's indentation level
        let mut lowest = usize::MAX;

        for i in self.indent_stack.iter().rev() {
            let i = *i;

            if spaces == i {
                break;
            } else if spaces < i && i < lowest {
                if self.indent_counter == 0 {
                    eprintln!("Indentation error (line {}): could not find line with matching indentation level within file.\n", self.token_location.0 + 1);
                    return;
                }

                lowest = i;
                self.indent_counter -= 1;

                self.tokens.push(Token::from(TokenKind::Dedent));
            }
        }

        self.indent_stack.push(spaces);
    }

    fn resolve_indentation_level(&mut self) {
        for _ in 0..self.indent_counter {
            self.tokens.push(Token::from(TokenKind::Dedent));
        }
    }

    fn get_identifier(&mut self) {
        let mut lexeme = String::new();

        while !self.reached_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            lexeme.push(self.advance());
        }

        let kind = match lexeme.as_str() {
            "func" => TokenKind::Func,
            "define" => TokenKind::Define,
            "return" => TokenKind::Return,
            "continue" => TokenKind::Continue,
            "break" => TokenKind::Break,
            "print" => TokenKind::Print,

            "and" => TokenKind::And,
            "or" => TokenKind::Or,
            "not" => TokenKind::Not,

            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "loop" => TokenKind::Loop,

            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "none" => TokenKind::None,

            _ => TokenKind::Identifier(lexeme.clone()),
        };

        self.tokens.push(Token::new(kind, lexeme));
    }

    fn get_number(&mut self) {
        let mut lexeme = String::new();
        let mut has_period = false;

        while !self.reached_end()
            && (self.peek().is_ascii_digit()
                || self.peek() == '_'
                || (self.peek() == '.' && !has_period))
        {
            if self.peek() == '.' {
                has_period = true;
            }

            lexeme.push(self.advance());
        }

        let number: f64 = lexeme.parse().unwrap();

        self.tokens
            .push(Token::new(TokenKind::Number(number), lexeme));
    }

    fn get_string(&mut self) {
        // Skip over preceding quotation mark
        self.advance();

        let mut lexeme = String::new();

        while !self.reached_end() && self.peek() != '"' && self.peek() != '\'' {
            let c = self.advance();

            if c == '\\' {
                let sc = match self.advance() {
                    '"' => '"',
                    '\'' => '\'',
                    '\\' => '\\',
                    '{' => '{',
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',

                    _ => {
                        lexeme.push('\\');
                        c
                    }
                };

                lexeme.push(sc);
            } else {
                lexeme.push(c);
            }
        }

        if self.reached_end() {
            eprintln!(
                "Lexing error (line {}): found EoF while looking for end of string.\n",
                self.token_location.0 + 1
            );
            return;
        }

        // Skip over trailing quotation mark
        self.advance();

        self.tokens
            .push(Token::new(TokenKind::String(lexeme.clone()), lexeme));
    }

    fn get_symbol(&mut self) {
        let c = self.advance();

        let kind = match c {
            // Single character tokens
            '(' => {
                self.bracket_counter += 1;
                Some(TokenKind::LeftParen)
            }
            ')' => {
                if self.paren_counter == 0 {
                    eprintln!("Bracket error (line {}): found closing parenthesis without matching opening parenthesis.\n", self.token_location.0 + 1);
                    self.contains_errors = true;
                } else {
                    self.paren_counter -= 1;
                }

                Some(TokenKind::RightParen)
            }
            '[' => {
                self.bracket_counter += 1;
                Some(TokenKind::LeftBracket)
            }
            ']' => {
                if self.bracket_counter == 0 {
                    eprintln!("Bracket error (line {}): found closing bracket without matching opening bracket.\n", self.token_location.0 + 1);
                    self.contains_errors = true;
                } else {
                    self.bracket_counter -= 1;
                }

                Some(TokenKind::RightBracket)
            }

            ',' => Some(TokenKind::Comma),
            '.' => Some(TokenKind::Period),
            ':' => Some(TokenKind::Colon),

            // Single and double character tokens
            '+' => {
                if self.consume_match('=') {
                    Some(TokenKind::PlusEqual)
                } else {
                    Some(TokenKind::Plus)
                }
            }
            '-' => {
                if self.consume_match('=') {
                    Some(TokenKind::MinusEqual)
                } else {
                    Some(TokenKind::Minus)
                }
            }
            '*' => {
                if self.consume_match('=') {
                    Some(TokenKind::StarEqual)
                } else {
                    Some(TokenKind::Star)
                }
            }
            '/' => {
                if self.consume_match('=') {
                    Some(TokenKind::SlashEqual)
                } else {
                    Some(TokenKind::Slash)
                }
            }
            '%' => {
                if self.consume_match('=') {
                    Some(TokenKind::PercentEqual)
                } else {
                    Some(TokenKind::Percent)
                }
            }
            '^' => {
                if self.consume_match('=') {
                    Some(TokenKind::CaretEqual)
                } else {
                    Some(TokenKind::Caret)
                }
            }

            '!' => {
                if self.consume_match('=') {
                    Some(TokenKind::BangEqual)
                } else {
                    Some(TokenKind::Bang)
                }
            }
            '=' => {
                if self.consume_match('=') {
                    Some(TokenKind::EqualEqual)
                } else {
                    Some(TokenKind::Equal)
                }
            }
            '>' => {
                if self.consume_match('=') {
                    Some(TokenKind::GreaterEqual)
                } else {
                    Some(TokenKind::Greater)
                }
            }
            '<' => {
                if self.consume_match('=') {
                    Some(TokenKind::LessEqual)
                } else {
                    Some(TokenKind::Less)
                }
            }

            // Control tokens
            '\n' => {
                self.at_new_line = true;

                self.location.0 += 1;
                self.location.1 = 0;

                // Only add a newline token if the previous token was not another newline token
                let prev = self.tokens.last();

                if let Some(prev) = prev {
                    if prev.kind == TokenKind::Newline {
                        None
                    } else {
                        Some(TokenKind::Newline)
                    }
                } else {
                    None
                }
            }

            _ => {
                eprintln!(
                    "Lexing error (line {}): found unknown symbol '{}' in file.\n",
                    self.token_location.0 + 1,
                    c
                );
                self.contains_errors = true;
                None
            }
        };

        if let Some(kind) = kind {
            self.tokens.push(Token::from(kind));
        }
    }

    pub fn collect_tokens(&mut self) -> Vec<Token> {
        self.reset();

        while !self.reached_end() {
            if self.at_new_line
                && !self.at_empty_line()
                && self.bracket_counter == 0
                && self.paren_counter == 0
            {
                self.get_indentation_level();
            }

            self.skip_whitespace();
            self.token_location = self.location;

            let c = self.peek();

            if c.is_alphabetic() || c == '_' {
                self.get_identifier();
            } else if c.is_ascii_digit() {
                self.get_number();
            } else if c == '"' || c == '\'' {
                self.get_string();
            } else if c == '#' {
                self.skip_line();
            } else {
                self.get_symbol();
            }
        }

        let prev = self.tokens.last();

        if let Some(prev) = prev {
            if prev.kind != TokenKind::Newline {
                self.tokens.push(Token::from(TokenKind::Newline));
            }
        }

        self.resolve_indentation_level();

        self.tokens.clone()
    }
}
