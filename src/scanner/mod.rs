use crate::{token::{Token, TokenType, TokenLiteralType}, errors::LoxErrors};

// TODO try to implement the error handling of multiple mistypen tokens
// * DONE rewrite methods names without camel_case 
// TODO handle basic errors

#[derive(Debug)]
pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
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

    pub fn scan_tokens(&mut self) -> Result<&str, LoxErrors> {
        while !self.isAtEnd() {
            let current_token = self.current_token();
            let new_token = self.scan_token(current_token)?;

            match new_token {
                Some(token) => self.tokens.push(token),
                None => ()
            }
            
            self.current += 1; // ? it's different from the book but I think it's alright
        }

        self.tokens.push(
            Token::new(
                TokenType::EOF,
                String::new(),
                TokenLiteralType::String(String::new()),
                self.line
            )
        );

        Ok("Tudo certo!")
    }

    fn scan_token(&mut self, c: char) -> Result<Option<Token>, LoxErrors> {
        let result =
        match c {
            ' ' | '\r' | '\t' => return Ok(None),
            '\n' => {
                self.line += 1;
                return Ok(None)
            }
            _ =>  
            Some(
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
                    },
                    '"' => {
                        while self.current_token() != '"' && !self.isAtEnd() {
                            if self.current_token() == '\n' {
                                self.line += 1;
                            }

                            self.advance();
                        }

                        if self.isAtEnd() {
                            return Err(LoxErrors::LexicalError { message: String::from("Unterminated string") })
                        }

                        self.add_new_token(
                            TokenType::STRING, 
                            Some(TokenLiteralType::String(String::from(&self.source[self.start..self.current]))))
                    },
                    _ => {
                        if c.is_digit(10) {
                            while self.peek().is_digit(10) {
                                self.advance();
                            }

                            if self.peek() == '.' && self.peek_next().is_digit(10) {
                                self.advance();

                                while self.peek().is_digit(10) {
                                    self.advance();
                                }
                            }
                        }

                        let my_float: f64 = match String::from(&self.source[self.start..self.current]).parse() {
                            Ok(val) => val,
                            Err(_) => return Err(LoxErrors::LexicalError { message: String::from("Something occured when trying to parse a String to float") })
                        };

                        self.add_new_token(
                            TokenType::NUMBER, 
                            Some(TokenLiteralType::Number(my_float)))
                    }
                }
            )
        };

        Ok(result)
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

    fn peek_next(&self) -> char {
        if self.isAtEnd() { return '\0'; }

        self.get_next_token()
    }

    fn current_token(&self) -> char {
        self.source.chars().nth(self.current).unwrap()
    }

    fn get_next_token(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn add_new_token(&self, token_type: TokenType, literal: Option<TokenLiteralType>) -> Token {
        let lexeme = &self.source[self.start..self.current]; // ! MAYBE this slice don't consider the start the right way

        Token::new(
            token_type,
            String::from(lexeme),
            match literal {
                Some(val) => val,
                None => TokenLiteralType::String(String::new())
            },
            0)
    }

    fn isAtEnd(&self) -> bool {
        self.current >= self.source.len() - 1
    }
}
