use std::io::{Error as IoError};
use std::fmt;

/// Custom error type for the Chak version control system
#[derive(Debug)]
pub enum ChakError {
    // System / I/O
    StdIoError(IoError),
    EnvVarNotFound(String),
    InvalidPath(String),

    // General-purpose
    CustomError(String),
    NoEntriesFound(String),

    //hash pointer
    InvalidHashLength(String),
    HashPointerNotFound(String),

    // Serialization
    SerializationError(String),
    DeserializationError(String),

    // Repo-specific
    RepoNotInitialized,
    RepoAlreadyExists,
    InvalidRepoState(String),
    MissingObject(String),
    NothingToCommit,
    FileNotTracked(String),
    MergeConflict(String),

    // Branch/ref-related
    BranchNotFound(String),
    InvalidBranchName(String),
    DetachedHead,
    CannotDeleteCurrentBranch,

    // Command/input-related
    InvalidCommand(String),
    MissingArgument(String),
    AmbiguousReference(String),

    // Locking/concurrency
    RepoLocked,
    ConcurrentOperationError,
}

impl fmt::Display for ChakError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // System / I/O
            ChakError::StdIoError(err) => write!(f, "I/O error: {}", err),
            ChakError::EnvVarNotFound(var) => write!(f, "Environment variable not found: {}", var),
            ChakError::InvalidPath(msg) => write!(f, "Invalid path: {}", msg),

            // General-purpose
            ChakError::CustomError(msg) => write!(f, "Error: {}", msg),
            ChakError::NoEntriesFound(msg) => write!(f, "No entries found: {}", msg),

            // Serialization
            ChakError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            ChakError::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),

            // Repo-specific
            ChakError::RepoNotInitialized => write!(f, "Repository not initialized"),
            ChakError::RepoAlreadyExists => write!(f, "Repository already exists"),
            ChakError::InvalidRepoState(msg) => write!(f, "Invalid repository state: {}", msg),
            ChakError::MissingObject(msg) => write!(f, "Missing object: {}", msg),
            ChakError::NothingToCommit => write!(f, "Nothing to commit"),
            ChakError::FileNotTracked(path) => write!(f, "File not tracked: {}", path),
            ChakError::MergeConflict(msg) => write!(f, "Merge conflict: {}", msg),

            // Branch/ref-related
            ChakError::BranchNotFound(branch) => write!(f, "Branch not found: {}", branch),
            ChakError::InvalidBranchName(name) => write!(f, "Invalid branch name: {}", name),
            ChakError::DetachedHead => write!(f, "HEAD is detached"),
            ChakError::CannotDeleteCurrentBranch => write!(f, "Cannot delete the current branch"),

            // Command/input-related
            ChakError::InvalidCommand(cmd) => write!(f, "Invalid command: {}", cmd),
            ChakError::MissingArgument(arg) => write!(f, "Missing argument: {}", arg),
            ChakError::AmbiguousReference(r) => write!(f, "Ambiguous reference: {}", r),

            // Locking/concurrency
            ChakError::RepoLocked => write!(f, "Repository is locked"),
            ChakError::ConcurrentOperationError => write!(f, "Another operation is already in progress"),
            ChakError::InvalidHashLength(hash) => write!(f, "Invalid hash length: {}", hash),
            ChakError::HashPointerNotFound(hash) => write!(f, "Hash pointer not found: {}", hash),
        }
    }
}

impl std::error::Error for ChakError {}

impl From<IoError> for ChakError {
    fn from(err: IoError) -> Self {
        ChakError::StdIoError(err)
    }
}
