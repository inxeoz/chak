use crate::hashing::HashPointer;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Function to create a directory and all its parent directories if they don't exist.
pub fn create_fold(dir: &Path) {
    fs::create_dir_all(dir).expect("Failed to create directory");
}

/// Function to create a file and write a message to it.
pub fn create_file_with_blob_pointer(parent_fold: &Path, blob_pointer: &HashPointer, contents: Option<Vec<u8>>) {
    // Create the full file path
    let file_path = parent_fold.join(blob_pointer.get_path());

    // Ensure the parent directory exists, create it if not
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                panic!("Failed to create parent directory {:?}: {}", parent, e);
            }
        }
    } else {
        panic!("Cannot get parent directory for file path: {:?}", file_path);
    }

    // Create and write to the file
    match File::create(&file_path) {
        Ok(mut file) => {
            if let Some(content) = contents {
                // Write content to the file
                if let Err(e) = file.write_all(&content) {
                    panic!("Failed to write to file {:?}: {}", file_path, e);
                }
            }

            println!("File {} created", file_path.display());
        }
        Err(e) => panic!("Failed to create file {:?}: {}", file_path, e),
    }
}

pub fn create_file(file_path: &PathBuf) {
    // Ensure the parent directory exists, create it if not
    if let Some(parent) = file_path.parent() {
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                panic!("Failed to create parent directory {:?}: {}", parent, e);
            }
        }
    } else {
        panic!("Cannot get parent directory for file path: {:?}", file_path);
    }
    // Create and write to the file
    match File::create(&file_path) {
        Ok(mut file) => {
            println!("File {} created", file_path.display());
        }
        Err(e) => panic!("Failed to create file {:?}: {}", file_path, e),
    }
}
/// Function to save content to a file.
//TODO fix saving anc creating
pub fn save_to_file(file_path: &Path, content: &str, append: bool) {

    if file_path.is_dir() || ! file_path.exists() {
        panic!("cant save content in file {}", file_path.display());
    }
    let file = if append {
        OpenOptions::new()
            .append(true) // Open the file in append mode
            .open(file_path)
    } else {
        OpenOptions::new()
            .write(true) // Open the file in write mode
            .truncate(true) // Truncate the file (clear its contents)
            .open(file_path)
    };

    match file {
        Ok(mut file) => {
            // Write the content to the file
            if let Err(e) = writeln!(file, "{}", content) {
                eprintln!("Error writing to file '{}': {}", file_path.display(), e);
            } else {
                println!("Successfully saved content to '{}'", file_path.display());
            }
        }
        Err(e) => eprintln!("Error opening file '{}': {}", file_path.display(), e),
    }
}

/// Function to get input from the command line.
pub fn input_from_commandline(prompt: &str) -> String {
    let mut input = String::new();

    // Print the prompt
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

    // Read user input
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase() // Return the trimmed and lowercase input
}

pub fn append_to_file(path: &Path, data: &str) -> io::Result<()> {
    // Open the file in append mode (create it if it doesn't exist)
    let mut file = OpenOptions::new()
        .append(true) // Open in append mode
        .create(true) // Create the file if it doesn't exist
        .open(path)?;
    // Write the data to the file
    writeln!(file, "{}", data)?; // Use `writeln!` to add a newline after the data
    Ok(())
}
