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
    // TODO IMPLEMENT ERROR HANDLING FOR SCANNER

    fn scanTokens(&mut self) {
        while !self.isAtEnd() {
            self.start = self.current;
            self.tokens.push(self.scanToken());
        }

        self.tokens.push(Token::new(TokenType::EOF, String::new(), String::new(), self.line));
    }

    fn scanToken(&self) -> Token {
        let c = self.advance();   

        match c {
            '(' => self.addNewToken(TokenType::LEFT_PAREN, None),
            ')' => self.addNewToken(TokenType::RIGHT_PAREN, None),
            '{' => self.addNewToken(TokenType::LEFT_BRACE, None),
            '}' => self.addNewToken(TokenType::RIGHT_BRACE, None),
            ',' => self.addNewToken(TokenType::COMMA, None),
            '.' => self.addNewToken(TokenType::DOT, None),
            '-' => self.addNewToken(TokenType::MINUS, None),
            '+' => self.addNewToken(TokenType::PLUS, None),
            ';' => self.addNewToken(TokenType::SEMICOLON, None),
            '*' => self.addNewToken(TokenType::STAR, None),
            _ => self.addNewToken(TokenType::EOF, None) // TODO do this return an error
        }
    }

    fn addNewToken(&self, token_type: TokenType, literal: Option<String>) -> Token {
        let lexeme = &self.source[self.start..self.current];

        Token::new(
            token_type,
            String::from(lexeme),
            match literal {
                Some(val) => val,
                None => String::new()
            },
            0)
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
