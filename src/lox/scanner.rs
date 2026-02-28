use crate::lox::tokens::Literal;

use super::tokens;

struct Scanner {
    source: Vec<u8>,
    tokens: Vec<tokens::Token>,
}

impl Scanner {
    pub fn new(source: Vec<u8>) -> Self {
        Self {
            source: source,
            tokens: Vec::new(),
        }
    }

    fn scan_tokens(&mut self) {
        let mut start = 0;
        let mut current = 0;
        let mut line = 1;
        while current < self.source.len() {
            start = current;
            self.scan_token(start, current, line);
        }
    }

    fn scan_token(&mut self, start: usize, current: usize, line: i32) {
        let c = self.get_lexem(start, current);
        if c == "(" {
            self.add_token(tokens::TokenType::LEFT_PAREN, c, line);
        } else if c == ")" {
            self.add_token(tokens::TokenType::RIGHT_PAREN, c, line);
        } else if c == "{" {
            self.add_token(tokens::TokenType::LEFT_BRACE, c, line);
        } else if c == "}" {
            self.add_token(tokens::TokenType::RIGHT_BRACE, c, line);
        } else if c == "," {
            self.add_token(tokens::TokenType::COMMA, c, line);
        } else if c == "." {
            self.add_token(tokens::TokenType::DOT, c, line);
        } else if c == "-" {
            self.add_token(tokens::TokenType::MINUS, c, line);
        } else if c == "+" {
            self.add_token(tokens::TokenType::PLUS, c, line);
        } else if c == ";" {
            self.add_token(tokens::TokenType::SEMICOLON, c, line);
        } else if c == "*" {
            self.add_token(tokens::TokenType::STAR, c, line);
        } else {
            crate::error(line, "Unexpected Character".into());
        }
    }

    // I was wrong, the bytes in the `source` vec must be consumed after being tokenized
    // this method is harzardous

    fn get_lexem(&self, start: usize, current: usize) -> String {
        String::from_utf8(self.source[start..current].to_vec()).unwrap() // brittle
    }

    fn add_token(&mut self, token_type: tokens::TokenType, lexeme: String, line: i32) {
        let token = tokens::Token::new(token_type, lexeme, line);
        self.tokens.push(token);
    }

    fn add_token_literal(
        &mut self,
        token_type: tokens::TokenType,
        lexeme: String,
        literal: Literal,
        line: i32,
    ) {
        let token = tokens::Token::new(token_type, lexeme, line).literal(literal);
        self.tokens.push(token);
    }
}
