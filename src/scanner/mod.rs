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

    fn new(src: String) -> Scanner {
        Scanner {
             source: src,
             tokens: Vec::new(),
             start: 0,
             current: 0,
             line: 0
        } 
     }

    fn scanTokens(&mut self) {
        while !self.isAtEnd() {
            let current_token = self.getCurrentToken();
            let new_token = self.scanToken(current_token);

            self.tokens.push(new_token);
            self.current += 1; // ? it's different from the book but I think it's alright
        }

        self.tokens.push(
            Token::new(
                TokenType::EOF,
                String::new(),
                String::new(),
                self.line
            )
        );
    }

    fn scanToken(&mut self, c: char) -> Token {
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
            '!' => {
                let new_token = if self.matchNextToken('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };

                self.addNewToken(new_token, None)
            },
            '=' => {
                let new_token = if self.matchNextToken('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };

                self.addNewToken(new_token, None)
            },
            '<' => {
                let new_token = if self.matchNextToken('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };

                self.addNewToken(new_token, None)
            },
            '>' => {
                let new_token = if self.matchNextToken('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };

                self.addNewToken(new_token, None)
            },
            _ => self.addNewToken(TokenType::EOF, None) // TODO do this return an error
        }
    }

    fn setStart(&mut self, new_start: usize) {
        self.start = new_start;
    }
    
    fn matchNextToken(&mut self, expected: char) -> bool {
        if self.isAtEnd() // ? this verification is not needed unless this function is used in other places
            || self.source.chars().nth(self.current + 1).unwrap() != expected {
            return false;
        }

        self.current += 1;
        
        true
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

    // ? the `advance` was refactored to this method
    // ? due to some problems with multiple mutable references of the struct
    fn getCurrentToken(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }

    fn isAtEnd(&self) -> bool {
        self.current >= self.source.len() - 1
    }
}
