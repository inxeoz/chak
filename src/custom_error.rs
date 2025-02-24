use std::io::{Error, ErrorKind};

#[derive(Debug)]
pub enum ChakError {
    StdIoError(Error),     // Store full Error, not just ErrorKind
    CustomError(String),   // Custom error
    NoEntriesFound,
}

use std::fmt;

impl fmt::Display for ChakError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChakError::StdIoError(err) => write!(f, "Standard I/O error: {}", err),
            ChakError::CustomError(msg) => write!(f, "Chak error: {}", msg),
            ChakError::NoEntriesFound => write!(f, "No entries found"),
        }
    }
}

impl std::error::Error for ChakError {}

// ✅ Fix: Store full `std::io::Error`
impl From<Error> for ChakError {
    fn from(error: Error) -> Self {
        ChakError::StdIoError(error)
    }
}
