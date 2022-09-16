use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
}

impl Scanner {
    fn scanTokens(&mut self) {
        while !self.isAtEnd() {
            self.start = self.current;
            self.scanTokens();
        }

        self.tokens.push(Token::new(TokenType::EOF, self.line));
    }

    fn new(src: String) -> Scanner {
       Scanner {
            source: src,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0
       } 
    }

    fn isAtEnd(&self) -> bool {
        self.current >= self.source.len()
    }
}
