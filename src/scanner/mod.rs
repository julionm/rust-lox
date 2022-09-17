use crate::token::{Token, TokenType};

// TODO try to implement the error handling of multiple mistypen tokens

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
            self.tokens.push(self.scanToken());
        }

        self.tokens.push(Token::new(TokenType::EOF, self.line));
    }

    fn scanToken(&self) -> Token {
        let c = self.advance();   

        match c {
            '(' => Token::new(TokenType::LEFT_PAREN, 0),
            _ => Token::new(TokenType::EOF, 0)
        }
    }

    fn advance(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
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
