use std::io::ErrorKind;

#[derive(Debug)]
pub enum ChakError {
    StdIoError(ErrorKind), // Reuse std::io::ErrorKind
    CustomError(String),   // Your custom error
}

use std::fmt;

impl fmt::Display for ChakError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChakError::StdIoError(kind) => write!(f, "Standard I/O error: {:?}", kind),
            ChakError::CustomError(msg) => write!(f, "chak error: {}", msg),
        }
    }
}

impl std::error::Error for ChakError {}

impl From<ErrorKind> for ChakError {
    fn from(kind: ErrorKind) -> Self {
        ChakError::StdIoError(kind)
    }
}