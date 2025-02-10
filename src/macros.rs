use crate::hashing::HashPointer;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{self, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};

/// Saves content to a file, creating it if it doesn't exist.
pub fn save_or_create_file(
    file_path: &Path,
    content: Option<&str>,
    append: bool
) -> io::Result<File> {
    if file_path.is_dir() {
        return Err(io::Error::new(
            ErrorKind::IsADirectory,
            "file path is a directory",
        ));
    }

    if let Some(parent_path) = file_path.parent() {
        fs::create_dir_all(parent_path)?;
    }
    let mut file = OpenOptions::new()
        .write(true)
        .append(append)
        .truncate(!append) // Truncate if not appending
        .create(true) // Create the file if it doesn't exist
        .open(file_path)?;

    if let Some(content) = content {
        file.write_all(content.as_bytes())?;
    }

    Ok(file) // Return Ok even if content is None
}

/// Function to get input from the command line.
pub fn input_from_commandline(prompt: &str) -> io::Result<String> {
    let mut input = String::new();

    if prompt.len() > 0 {
        print!("{}", prompt);
        io::stdout().flush()?; // Ensure the prompt is displayed immediately
    }

    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_lowercase())
}

pub fn file_to_string(file: &mut File) -> io::Result<String> {
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
