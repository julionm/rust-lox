use std::env;
use std::io::{Read, stdin};
use std::fs::File;

// TODO study more custom errors to handle better

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
