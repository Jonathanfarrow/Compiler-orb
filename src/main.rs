mod scanner;
mod errors;

use crate :: scanner::*;
use crate :: errors::MyError;

use std::io::{self, Write};
use std::{env, fs, vec};
use std::cell::Cell;
use std::process::exit;
use std::error::Error;


thread_local! {
    static HAD_ERROR: Cell<bool> = Cell::new(false);
}






fn run_file(path: &str) -> Result<(),MyError> {
    let contents = fs::read_to_string(path).map_err(|e| MyError::FileReadError {
        path: path.to_string(),
        error: e,
    })?;
    println!("File contents: {}", contents);
    run(&contents)?;
    Ok(())

}


fn run(_contents: &str) -> Result<Vec<Token>, MyError> {
    let mut scanner = Scanner::new(_contents);
    
    // Scan tokens and handle potential errors
    match scanner.scan_tokens() {
        Ok(tokens) => {
            for token in &tokens {
                println!("Token: {:?}", token);
            }
            Ok(tokens)
        }
        Err(e) => {
           
            Err(e)
        }
    }
}

// Function to handle the interactive prompt
fn run_prompt() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();

    loop {
        let mut buffer = String::new();
        print!("Enter input (or type 'exit' to quit): ");
        io::stdout().flush()?; // Flush the standard output to ensure prompt is displayed

        stdin.read_line(&mut buffer)?;

        let input = buffer.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        if input.is_empty() {
            continue;
        }

        println!("You wrote: {}", input);
        
        if let Err(e) = run(input) {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}


fn main() {
    let args: Vec<String> = env::args().collect();


    match args.len() {
        1 => {
            if let Err(e) = run_prompt() {
                eprintln!("Error during interactive prompt: {}", e);
                exit(1);
            }
            } // When there are no additional arguments
        2 => {
            if let Err(e) = run_file(&args[1]) {
                eprintln!(" {}", e);
                exit(1);
            }
        },
        _ => {
            eprintln!("Usage: jloc [script]");
            exit(64);
        }
    }
}
