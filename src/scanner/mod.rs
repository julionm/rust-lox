use crate::{token::{Token, TokenType, TokenLiteralType}, errors::LoxErrors};
use std::collections::HashMap;

// TODO try to implement the error handling of multiple mistypen tokens
// TODO handle basic errors

#[derive(Debug)]
pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,

    // TODO once OnceCell is released, I should replace all the IDENTIFIERS HashMap
    identifiers: HashMap<String, TokenType>
}

impl Scanner{
    // TODO IMPLEMENT ERROR HANDLING FOR SCANNER

    pub fn new(src: String) -> Scanner {
        Scanner {
             source: src,
             tokens: Vec::new(),
             start: 0,
             current: 0,
             line: 0,
             identifiers: HashMap::from([
                (String::from("or"), TokenType::OR),
                (String::from("and"), TokenType::AND),
                (String::from("class"), TokenType::CLASS),
                (String::from("else"), TokenType::ELSE),
                (String::from("false"), TokenType::FALSE),
                (String::from("for"), TokenType::FOR),
                (String::from("fun"), TokenType::FUN),
                (String::from("if"), TokenType::IF),
                (String::from("nil"), TokenType::NIL),
                (String::from("print"), TokenType::PRINT),
                (String::from("return"), TokenType::RETURN),
                (String::from("super"), TokenType::SUPER),
                (String::from("this"), TokenType::THIS),
                (String::from("true"), TokenType::TRUE),
                (String::from("var"), TokenType::VAR),
                (String::from("while"), TokenType::WHILE)
             ])
        } 
    }

    pub fn scan_tokens(&mut self) -> Result<&str, LoxErrors> {

        // ? As it is now, the EOF token MUST be at the sample file to work well
        while !self.is_at_end() {
            self.start = self.current;

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
                            TokenType::EQUAL_EQUAL
                        } else {
                            TokenType::EQUAL
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
                            while self.current_token() != '\n' && !self.is_at_end() {
                                self.advance();
                            }
                        }

                        self.add_new_token(TokenType::EOF, None)
                    },
                    '"' => {
                        self.advance();

                        while self.current_token() != '"' && !self.is_at_end() {
                            if self.current_token() == '\n' {
                                self.line += 1;
                            }

                            self.advance();
                        }

                        if self.is_at_end() {
                            return Err(LoxErrors::LexicalError { message: String::from("Unterminated string") })
                        }

                        self.add_new_token(
                            TokenType::STRING, 
                            Some(TokenLiteralType::String(String::from(&self.source[self.start..self.current+1]))))
                    },
                    _ => {
                        if c.is_digit(10) {
                            while self.peek_next().is_digit(10) {
                                self.advance();
                            }

                            if self.peek_next() == '.' {
                                self.advance();

                                if self.peek_next().is_digit(10) {
                                    self.advance();
    
                                    while self.peek_next().is_digit(10) {
                                        self.advance();
                                    }
                                } else if self.current_token() == '.' {
                                    return Err(LoxErrors::LexicalError { message: String::from("Malformed Number, for float point use 123.123") });
                                }
                            }

                            let my_float: f64 = match String::from(&self.source[self.start..self.current+1]).parse() {
                                Ok(val) => val,
                                Err(err) => return Err(
                                    LoxErrors::LexicalError { 
                                        message: format!("An Unexpected Error Occured: {}", err.to_string()) 
                                    })
                            };
    
                            self.add_new_token(
                                TokenType::NUMBER, 
                                Some(TokenLiteralType::Number(my_float)))
                        } else if c.is_alphabetic() {
                            // TODO implement this
                            while self.peek_next().is_alphanumeric() {
                                self.advance();
                            }
                            
                            let ident = String::from(&self.source[self.start..self.current+1]);

                            match self.identifiers.get(&ident) {
                                Some(value) => 
                                    self.add_new_token(
                                        value.clone(),
                                        Some(TokenLiteralType::Identifier(ident)))
                                        ,
                                None => 
                                    self.add_new_token(
                                        TokenType::IDENTIFIER, 
                                        Some(TokenLiteralType::Identifier(ident)))
                            }
                        } else {
                            return Err(LoxErrors::LexicalError { message: format!("Unindentified character: {}", c) });
                        }

                        
                    }
                }
            )
        };

        Ok(result)
    }

    fn consume_next_token(&mut self, expected: char) -> bool {
        if self.is_at_end() // ? this verification is not needed unless this function is used in other places
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

    fn peek_next(&self) -> char {
        if self.is_at_end() { return '\0'; }

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
            self.line)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() - 1
    }
}
