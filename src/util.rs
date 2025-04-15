use std::any::{Any, type_name};
use std::fs::read_dir;

use crate::chak_traits::HashPointerTraits;
use crate::config::{CHAK_FOLDER_NAME, REGISTER_NAME};
use crate::custom_error::ChakError;
use crate::hash_pointer::HashPointer;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};

pub fn path_buf_to_name(path: &Path) -> Result<String, ChakError> {
    Ok(path
        .file_name()
        .ok_or(ChakError::CustomError(
            "Could not get file name".to_string(),
        ))?
        .to_str()
        .ok_or(ChakError::CustomError(
            "Could not convert to str".to_string(),
        ))?
        .to_string())
}

pub fn path_buf_to_parent_and_name(path: &Path) -> Result<(PathBuf, String), ChakError> {
    Ok((
        path.parent()
            .ok_or(ChakError::CustomError(
                "Parent of entry not found".to_string(),
            ))?
            .to_path_buf(),
        path_buf_to_name(path)?,
    ))
}

pub fn deserialize_file_content<T: DeserializeOwned>(path: &Path) -> Result<T, ChakError> {
    let content_string = fs::read_to_string(path)?; // Reads file, propagates error if any

    if let Ok(content) = toml::from_str(&content_string) {
        Ok(content)
    } else {
        Err(ChakError::DeserializationError(format!(
            " in path {} ",
            path.display().to_string()
        )))
    }
}

pub fn serialize_struct<T: Serialize>(data: &T) -> Result<String, ChakError> {
    let serialized = toml::to_string(data).map_err(|e| {
        ChakError::SerializationError(
            format!("serialization failed for {}", type_name::<T>()).to_string(),
        )
    })?;

    Ok(serialized)
}

pub fn serialize_and_save<T: Serialize>(data: &T, path_to_save: &Path) -> Result<File, ChakError> {
    let serialized = serialize_struct(data)?;
    save_or_create_file(path_to_save, Some(&serialized.as_str()), false, None)
}

pub fn check_vcs_presence_in_dir(fold: &Path) -> bool {
    if fold.join(CHAK_FOLDER_NAME).exists() {
        return true;
    }

    // Read the directory and check subdirectories recursively
    if let Ok(entries) = read_dir(fold) {
        for entry in entries {
            if let Ok(entry) = entry {
                // Recursively check each subdirectory
                return check_vcs_presence_in_dir(&entry.path());
            }
        }
    }
    false
}

pub fn read_directory_entries(path: &Path) -> Result<(Vec<PathBuf>, Vec<PathBuf>), ChakError> {
    let entries = read_dir(path)?;
    let mut detected_dir_entries = Vec::new();
    let mut detected_file_entries = Vec::new();
    for entry in entries {
        let entry = entry?.path();
        if entry.is_dir() {
            detected_dir_entries.push(entry);
        } else {
            detected_file_entries.push(entry);
        }
    }
    Ok((detected_dir_entries, detected_file_entries))
}

/// Saves content to a file, creating it if it doesn't exist.
///

pub fn save_or_create_file(
    file_path: &Path,
    content: Option<&str>,
    append: bool,
    append_with_separator: Option<&str>,
) -> Result<File, ChakError> {
    if file_path.is_dir() {
        return Err(ChakError::StdIoError(io::Error::new(
            ErrorKind::IsADirectory,
            "path is a directory, not a file",
        )));
    }

    if let Some(parent_path) = file_path.parent() {
        fs::create_dir_all(parent_path)?;
    } else {
        return Err(ChakError::StdIoError(io::Error::new(
            ErrorKind::NotFound,
            "parent directory could not be determined",
        )));
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(append)
        .truncate(!append)
        .create(true)
        .open(file_path)?;

    if let Some(content) = content {
        file.write_all(content.as_bytes())?;
        if let Some(sep) = append_with_separator {
            file.write_all(sep.as_bytes())?;
        }
    }

    Ok(file)
}

/// Function to get input from the command line.
pub fn input_from_commandline(prompt: &str) -> Result<String, ChakError> {
    let mut input = String::new();

    if prompt.len() > 0 {
        print!("{}", prompt);
        io::stdout().flush()?; // Ensure the prompt is displayed immediately
    }

    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_lowercase())
}

pub fn file_to_string(file: &mut File) -> Result<String, ChakError> {
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn file_to_lines(file: &File) -> Vec<String> {
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.unwrap_or_default())
        .collect()
}

pub fn string_content_to_string_vec(content: &str) -> Vec<String> {
    content.lines().map(|s| s.trim().to_string()).collect()
}

pub fn was_it_registered<T: HashPointerTraits>(pointer: T, dir: &Path) -> bool {
    if let Ok(register) = File::open(dir.join(REGISTER_NAME)) {
        for line in file_to_lines(&register) {
            return line.trim() == pointer.get_one_hash();
        }
    }
    false
}
#[cfg(test)]
pub mod tests {
    use crate::config::get_current_dir_path;
    use crate::util::save_or_create_file;

    #[test]
    pub fn test_save_or_create() {
        save_or_create_file(
            &get_current_dir_path().join("test.txt"),
            Some("i am"),
            false,
            None,
        )
        .unwrap();
        save_or_create_file(
            &get_current_dir_path().join("test.txt"),
            Some("i am"),
            true,
            Some("\n"),
        )
        .unwrap();
    }
}
