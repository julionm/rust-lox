use std::fmt::Display;

pub enum TokenType {
    // ? single-character tokens
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // ? one or two character tokens
    BANG, BANG_EQUAL, EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL, LESS, LESS_EQUAL,

    // ? literals
    IDENTIFIER, STRING, NUMBER,

    //? keywords
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF,
    NIL, OR, PRINT, RETURN, SUPER, THIS,
    TRUE, VAR, WHILE,
    EOF
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "Token Type zika") // TODO improve this Display fmt
    }
}


pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: usize
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: usize) -> Token {
        Token {
            token_type,
            lexeme: String::new(),
            literal: String::new(),
            line
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}
