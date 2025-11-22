use crate::token::{LiteralType, Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            LiteralType::Nil,
            self.current,
        ));

        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        println!("todo: scan token");
        self.current += 1;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
