// TODO create more specific errors

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
