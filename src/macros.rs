use crate::hashing::HashPointer;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{self, ErrorKind, Write};
use std::path::{Path, PathBuf};

pub fn create_file_with_blob_pointer(
    parent_fold: &Path,
    blob_pointer: &HashPointer,
    contents: Option<Vec<u8>>,
) -> io::Result<()> {
    let final_parent_fold = parent_fold.join(blob_pointer.get_fold_name());
    let file_path = final_parent_fold.join(blob_pointer.get_file_name());

    fs::create_dir_all(final_parent_fold)?;
    let mut file = File::create(&file_path)?;


    if let Some(content) = contents {
        file.write_all(&content)?;
    }

    println!("File {} created", file_path.display());

    Ok(())
}

pub fn create_file<P: AsRef<Path>>(file_path: P) -> io::Result<()> {
    let file_path = file_path.as_ref();
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }
    File::create(file_path)?;
    Ok(())
}

/// Function to save content to a file.
//TODO fix saving anc creating
pub fn save_to_file(file_path: &Path, content: &str, append: bool) -> io::Result<()> {
    if !file_path.exists() {
        create_file(file_path)?;
    }
    if file_path.is_dir() {
        return Err(io::Error::new(
            ErrorKind::IsADirectory,
            "file path is a directory",
        ));
    }
    let mut file = if append {
        OpenOptions::new()
            .append(true) // Open the file in append mode
            .open(file_path)?
    } else {
        OpenOptions::new()
            .write(true) // Open the file in write mode
            .truncate(true) // Truncate the file (clear its contents)
            .open(file_path)?
    };

    writeln!(file, "{}", content)?;
    Ok(())
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
