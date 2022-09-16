use std::io::{Read, stdin};
use std::fs::File;

pub enum LoxErrors {
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

pub fn run_prompt() -> Result<String, LoxErrors> {

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

pub fn run_file<'a>(path: String) -> Result<&'a str, LoxErrors> {
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

pub fn run<'a>(code: String) -> Result<&'a str, LoxErrors>{
    println!("my code was: {code}");

    Ok("")
}

pub fn handle_error(error: LoxErrors) {
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
