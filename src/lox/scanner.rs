use super::tokens as toe;
use super::tokens::TokenType;

struct Scanner {
    source: String,
    tokens: Vec<toe::Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source: source,
            tokens: Vec::new(),
        }
    }

    fn scan_toe(&mut self) {
        let mut start = 0;
        let mut current = 0;
        let mut line = 1;
        while current < self.source.len() {
            start = current;
            current = self.scan_token(start, current, line);
        }
    }

    fn scan_token(&mut self, start: usize, current: usize, line: i32) -> usize {
        let mut c = get_lexem(&self.source, start, current);
        let token_type = if c == "(" {
            Some(TokenType::LEFT_PAREN)
        } else if c == ")" {
            Some(TokenType::RIGHT_PAREN)
        } else if c == "{" {
            Some(TokenType::LEFT_BRACE)
        } else if c == "}" {
            Some(TokenType::RIGHT_BRACE)
        } else if c == "," {
            Some(TokenType::COMMA)
        } else if c == "." {
            Some(TokenType::DOT)
        } else if c == "-" {
            Some(TokenType::MINUS)
        } else if c == "+" {
            Some(TokenType::PLUS)
        } else if c == ";" {
            Some(TokenType::SEMICOLON)
        } else if c == "*" {
            Some(TokenType::STAR)
        } else if c == "!" {
            if match_next(&self.source, current, "=") {
                c = get_lexem(&self.source, start, current + 1);
                Some(TokenType::BANG_EQUAL)
            } else {
                Some(TokenType::BANG)
            }
        } else if c == "=" {
            if match_next(&self.source, current, "=") {
                c = get_lexem(&self.source, start, current + 1);
                Some(TokenType::EQUAL_EQUAL)
            } else {
                Some(TokenType::EQUAL)
            }
        } else if c == ">" {
            if match_next(&self.source, current, "=") {
                c = get_lexem(&self.source, start, current + 1);
                Some(TokenType::GREATER_EQUAL)
            } else {
                Some(TokenType::GREATER)
            }
        } else if c == "<" {
            if match_next(&self.source, current, "=") {
                c = get_lexem(&self.source, start, current + 1);
                Some(TokenType::LESS_EQUAL)
            } else {
                Some(TokenType::LESS)
            }
        } else {
            None
        };

        if let Some(token_type) = token_type {
            add_token(&mut self.tokens, token_type, c.to_string(), line);
        } else {
            crate::error(line, "Unexpected symbol.".into()); // no way to track which erros occured
        }

        c.len()
    }
}

fn add_token(tokens: &mut Vec<toe::Token>, token_type: TokenType, lexeme: String, line: i32) {
    let token = toe::Token::new(token_type, lexeme, line);
    tokens.push(token);
}

fn add_token_literal(
    tokens: &mut Vec<toe::Token>,
    token_type: TokenType,
    lexeme: String,
    literal: toe::Literal,
    line: i32,
) {
    let token = toe::Token::new(token_type, lexeme, line).literal(literal);
    tokens.push(token);
}

// I think returning a ref will provide more flexibility,
// the caller can choose if they wish to clone or not
fn get_lexem(source: &String, start: usize, current: usize) -> &str {
    &source[start..current]
}

fn match_next(source: &String, current: usize, expected: &str) -> bool {
    if current >= source.len() {
        false
    } else if &source[current..current + 1] != expected {
        false
    } else {
        true
    }
}
