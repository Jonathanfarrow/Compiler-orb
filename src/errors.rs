use std::fmt;

#[derive(Debug)]
pub enum MyError {
    CommandLineArgs,
    FileReadError { path: String, error: std::io::Error },
    ParsingError(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::CommandLineArgs => {
                write!(f, "Please provide a file name as a command-line argument.")
            }
            MyError::FileReadError { path, error } => {
                write!(f, "Error reading the file '{}': {}", path, error)
            }
            MyError::ParsingError(msg) => write!(f, "Parsing error: {}", msg),
        }
    }
}

impl std::error::Error for MyError {}
