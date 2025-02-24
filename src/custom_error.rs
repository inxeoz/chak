use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub enum ChakError {
    StdIoError(ErrorKind), // Using ErrorKind
    CustomError(String),   // Your custom error
    NoEntriesFound,
}

use std::fmt;

impl fmt::Display for ChakError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChakError::StdIoError(kind) => write!(f, "Standard I/O error: {:?}", kind),
            ChakError::CustomError(msg) => write!(f, "Chak error: {}", msg),
            ChakError::NoEntriesFound => write!(f, "No entries found"),
        }
    }
}

impl std::error::Error for ChakError {}

// ✅ Fix: Implement From<std::io::Error>
impl From<Error> for ChakError {
    fn from(error: Error) -> Self {
        ChakError::StdIoError(error.kind()) // Extract ErrorKind
    }
}
