use crate::token::{Token, TokenType};

// TODO try to implement the error handling of multiple mistypen tokens
// * DONE rewrite methods names without camel_case 
// TODO handle basic errors

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
}

impl Scanner {
    // TODO IMPLEMENT ERROR HANDLING FOR SCANNER

    pub fn new(src: String) -> Scanner {
        Scanner {
             source: src,
             tokens: Vec::new(),
             start: 0,
             current: 0,
             line: 0
        } 
     }

    pub fn scan_tokens(&mut self) {
        while !self.isAtEnd() {
            let current_token = self.current_token();
            let new_token = self.scan_token(current_token);

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

    fn scan_token(&mut self, c: char) -> Token {
        match c {
            '(' => self.add_new_token(TokenType::LEFT_PAREN, None),
            ')' => self.add_new_token(TokenType::RIGHT_PAREN, None),
            '{' => self.add_new_token(TokenType::LEFT_BRACE, None),
            '}' => self.add_new_token(TokenType::RIGHT_BRACE, None),
            ',' => self.add_new_token(TokenType::COMMA, None),
            '.' => self.add_new_token(TokenType::DOT, None),
            '-' => self.add_new_token(TokenType::MINUS, None),
            '+' => self.add_new_token(TokenType::PLUS, None),
            ';' => self.add_new_token(TokenType::SEMICOLON, None),
            '*' => self.add_new_token(TokenType::STAR, None),
            '!' => {
                let new_token = if self.consume_next_token('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };

                self.add_new_token(new_token, None)
            },
            '=' => {
                let new_token = if self.consume_next_token('=') {
                    TokenType::BANG_EQUAL
                } else {
                    TokenType::BANG
                };

                self.add_new_token(new_token, None)
            },
            '<' => {
                let new_token = if self.consume_next_token('=') {
                    TokenType::LESS_EQUAL
                } else {
                    TokenType::LESS
                };

                self.add_new_token(new_token, None)
            },
            '>' => {
                let new_token = if self.consume_next_token('=') {
                    TokenType::GREATER_EQUAL
                } else {
                    TokenType::GREATER
                };

                self.add_new_token(new_token, None)
            },
            '/' => {
                if self.consume_next_token('/') {
                    while self.current_token() != '\n' && !self.isAtEnd() {
                        self.advance();
                    }
                }

                self.add_new_token(TokenType::EOF, None)
            }
            _ => self.add_new_token(TokenType::EOF, None) // TODO do this return an error
        }
    }

    fn set_start(&mut self, new_start: usize) {
        self.start = new_start;
    }
    
    fn consume_next_token(&mut self, expected: char) -> bool {
        if self.isAtEnd() // ? this verification is not needed unless this function is used in other places
            || self.source.chars().nth(self.current + 1).unwrap() != expected {
            return false;
        }

        self.current += 1;
        
        true
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.current_token()
    }
    
    fn peek(&self) -> char {
        if self.isAtEnd() { return '\0'; }

        self.current_token()
    }

    fn current_token(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }

    fn add_new_token(&self, token_type: TokenType, literal: Option<String>) -> Token {
        let lexeme = &self.source[self.start..self.current]; // ! MAYBE this slice don't consider the start the right way

        Token::new(
            token_type,
            String::from(lexeme),
            match literal {
                Some(val) => val,
                None => String::new()
            },
            0)
    }

    fn isAtEnd(&self) -> bool {
        self.current >= self.source.len() - 1
    }
}
