use std::env;
use std::io::{Read, stdin};
use std::fs::File;
use std::fmt::Display;

// TODO study more custom errors to handle better

enum TokenType {
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

struct Scanner {
    source: String,
    tokens: Vec<Token>
}

impl Scanner {
    fn scanTokens() { }
}

struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: u16
}

impl Token {
    fn new() -> Token {
        Token {
            token_type: TokenType::AND,
            lexeme: String::new(),
            literal: String::new(),
            line: 0
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

enum LoxErrors {
    ParserError {
        message: String
    },
    LexicalError {
        message: String
    },
    RuntimeError {
        message: String
    },
    FileOpenningError,
    FileReadingError
}

fn run_prompt() -> Result<String, LoxErrors> {

    let mut code = String::new();

    match stdin().read_line(&mut code) {
        Ok(_) => (),
        Err(err) => return Err(LoxErrors::FileOpenningError)
    };

    match run(code) {
        Ok(_) => {},
        Err(err) => return Err(err)
    };

    Ok(String::new())
}

fn run_file<'a>(path: String) -> Result<&'a str, LoxErrors> {
    let mut content = String::new();

    match File::open(path) {
        Ok(mut file) => {
            match file.read_to_string(&mut content) {
                Ok(_) => (),
                Err(_) => return Err(LoxErrors::FileReadingError)
            };
        },
        Err(_) => return Err(LoxErrors::FileOpenningError)
    }

    run(content)?; // TODO handle this

    Ok("")
}

fn run<'a>(code: String) -> Result<&'a str, LoxErrors>{
    println!("my code was: {code}");

    Ok("")
}

fn handle_error(error: LoxErrors) {
    match error {
        LoxErrors::FileOpenningError => {
            println!("Error trying to open the source code.");
        },
        LoxErrors::FileReadingError => {
            println!("Failed to read the source code.");
        },
        LoxErrors::LexicalError { message }
        | LoxErrors::ParserError { message }
        | LoxErrors::RuntimeError { message } => {
            println!("The interpreter crashed with the error: {}", message);
        }
    }
}

fn main() {
    let mut args_iter = env::args();
    args_iter.next(); // Skipping some environment vars

    let file_to_execute = args_iter.next();

    match file_to_execute {
        Some(file_path) => {
            match run_file(file_path) {
                Ok(_) => (), // TODO try to remember if it's possible to just ignore this
                Err(err) => handle_error(err)
            };
        },
        None => {
            // TODO implement the prompt later
            match run_prompt() {
                Ok(_) => {
                    println!("Finished execution.");
                },
                Err(err) => handle_error(err)
            }
        }
    }
}
