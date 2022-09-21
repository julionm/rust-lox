use std::io::{Read, stdin};
use std::fs::File;

use crate::errors::LoxErrors;
use crate::scanner::Scanner;

pub fn run_prompt() -> Result<String, LoxErrors> {

    let mut code = String::new();

    match stdin().read_line(&mut code) {
        Ok(_) => (),
        Err(_) => return Err(LoxErrors::FileOpenningError)
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
    
    let mut scanner = Scanner::new(code);

    scanner.scan_tokens()?;

    println!("{:?}", scanner.tokens);

    Ok("The scanning was alright")
}
