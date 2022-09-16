mod token;
mod scanner;
mod runtime;

use std::env;
use runtime::{
    handle_error,
    run_file,
    run_prompt
};

// TODO study more custom errors to handle better

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
