
use std::fs::read_dir;

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{self, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};

pub fn check_vcs_presence(fold: &Path) -> bool {
        if fold.join(".chak").exists() {
            return true;
        }
        // Read the directory and check subdirectories recursively
        if let Ok(entries) = read_dir(fold) {
            for entry in entries {
                if let Ok(entry) = entry {
                    // Recursively check each subdirectory
                    return check_vcs_presence(&entry.path())
                }
            }
        }
    false
}

pub fn read_directory_entries(path: &Path) -> io::Result<Vec<PathBuf>> {
    let entries = read_dir(path)?;
    let mut detected_entries = Vec::new();

    for entry in entries {
        let entry = entry?.path();
        detected_entries.push(entry.clone());
    }
    Ok(detected_entries)
}



/// Saves content to a file, creating it if it doesn't exist.
pub fn save_or_create_file(
    file_path: &Path,
    content: Option<&str>,
    append: bool,
    append_with_separator: Option<&str>,
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
        if let Some(sep_string) = append_with_separator {
            file.write_all(sep_string.as_bytes())?;
        }
        file.write_all(content.as_bytes())?;
    }

    file.sync_all()?;//lets see
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

pub fn string_content_to_string_vec(content: &str) -> Vec<String> {
    content
        .lines()
        .map(|s| s.trim().to_string())
        .collect()
}


#[cfg(test)]
pub mod tests {
    use crate::config::get_project_dir;
    use crate::util::save_or_create_file;

    #[test]
    pub fn test_save_or_create() {
        save_or_create_file(&get_project_dir().join("test.txt"), Some("i am"), false, None).unwrap();
        save_or_create_file(&get_project_dir().join("test.txt"), Some("i am"), true, Some("\n")).unwrap();
    }
}
