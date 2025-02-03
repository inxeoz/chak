use crate::config::MIN_HASH_LENGTH;
use crate::custom_error::ChakError;
use crate::macros::{create_file, create_file_with_blob_pointer};
use crate::tree_object::{TreeNode, TreeObject};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct HashPointer {
    fold_name: String,
    file_name: String,
}

impl PartialEq<Self> for HashPointer {
    fn eq(&self, other: &Self) -> bool {
        self.get_one_hash() == other.get_one_hash()
    }
}

impl PartialOrd<Self> for HashPointer {
    fn partial_cmp(&self, other: &Self) -> std::option::Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HashPointer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fold_name
            .cmp(&other.fold_name)
            .then_with(|| self.file_name.cmp(&other.file_name)) // Sort by file_name if fold_name is the same
    }
}

impl HashPointer {
    pub fn replace(&mut self, hash_pointer: &HashPointer) {
        self.fold_name = hash_pointer.get_fold_name();
        self.file_name = hash_pointer.get_file_name();
    }

    pub fn update_hash(&mut self, content: String) {
        let has_p = hash_combine(&self, &hash_from_content(content));
        self.replace(&has_p);
    }

    pub fn get_fold_name(&self) -> String {
        self.fold_name.clone()
    }

    pub fn get_file_name(&self) -> String {
        self.file_name.clone()
    }

    pub fn get_one_hash(&self) -> String {
        self.fold_name.clone() + &self.file_name
    }

    pub fn get_path(&self) -> PathBuf {
        PathBuf::from(&self.fold_name).join(&self.file_name)
    }
}

fn _hash_pointer_from_hash_string(hash: String) -> HashPointer {
    HashPointer {
        fold_name: hash[..2].to_string(),
        file_name: hash[2..].to_string(),
    }
}

pub fn hash_pointer_from_hash_string(hash: String) -> Result<HashPointer, ChakError> {
    if hash.len() < MIN_HASH_LENGTH {
        return Err(ChakError::CustomError(
            "Invalid hash string length".to_string(),
        ));
    }
    Ok(_hash_pointer_from_hash_string(hash))
}

pub fn hash_combine(first: &HashPointer, second: &HashPointer) -> HashPointer {
    hash_from_content(first.get_one_hash() + &second.get_one_hash())
}

pub fn hash_from_file(path: &Path) -> io::Result<HashPointer> {
    let file = File::open(path)?;
    let mut buf_file = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 1024];

    while let Ok(bytes_read) = buf_file.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Ok(_hash_pointer_from_hash_string(format!(
        "{:x}",
        hasher.finalize()
    )))
}

pub fn hash_from_save_blob(file_path: &Path, save_dir: &Path) -> io::Result<HashPointer> {
    let hash_pointer = hash_from_file(file_path)?;
    let blob_file = save_dir.join(hash_pointer.get_path());
    if blob_file.exists() {
        println!("file already exists");
        return Ok(hash_pointer);
    }

    let content = fs::read(file_path)?;
    create_file_with_blob_pointer(save_dir, &hash_pointer, Some(content))?;
    Ok(hash_pointer)
}
pub fn hash_from_save_content(save_dir: &Path, content: String) -> io::Result<HashPointer> {
    let hash_pointer = hash_from_content(content.clone());
    if !save_dir.join(hash_pointer.get_path()).exists() {
        create_file_with_blob_pointer(save_dir, &hash_pointer, Some(content.into_bytes()))?;
    }
    Ok(hash_pointer)
}

pub fn hash_from_pointers(pointers: Vec<HashPointer>) -> Result<HashPointer, ChakError> {
    if pointers.is_empty() {
        return Err(ChakError::CustomError(
            "Empty hash pointer vector".to_string(),
        ));
    }
    let mut hasher = Sha256::new();
    for pointer in pointers {
        hasher.update(pointer.get_one_hash().as_bytes());
    }
    Ok(_hash_pointer_from_hash_string(format!(
        "{:x}",
        hasher.finalize()
    )))
}

pub fn hash_from_content(content: String) -> HashPointer {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    _hash_pointer_from_hash_string(format!("{:x}", hasher.finalize()))
}

pub fn hash_from_string_vec(strings: &[String]) -> HashPointer {
    let mut hasher = Sha256::new();
    for string in strings {
        hasher.update(string.as_bytes());
    }
    _hash_pointer_from_hash_string(format!("{:x}", hasher.finalize()))
}

pub fn hash_from_save_tree(
    save_dir: &Path,
    children: IndexMap<String, TreeNode>,
) -> io::Result<HashPointer> {
    let mut tree_object = TreeObject { children };
    tree_object.sort_children();
    let content = serde_json::to_string(&tree_object)?;
    hash_from_save_content(save_dir, content)
}

pub fn get_latest_pointer_from_file(
    file_path: &Path,
    from_bottom: bool,
) -> Result<HashPointer, ChakError> {
    // Open the file and handle any I/O error by converting it into your custom error
    let file = File::open(file_path).map_err(|e| ChakError::StdIoError(e.kind()))?;
    let reader = BufReader::new(file);

    // Collect all lines into a vector of Strings, handling any I/O errors that might occur
    let lines: Result<Vec<String>, io::Error> = reader.lines().collect();
    let lines = lines.map_err(|e| ChakError::StdIoError(e.kind()))?;

    // Select the first or last line based on the from_bottom flag
    let hps = if from_bottom {
        lines.last()
    } else {
        lines.first()
    };

    // Try to create a HashPointer from the selected line
    match hps {
        Some(hash) => hash_pointer_from_hash_string(hash.clone()), // Clone the hash and process it
        None => Err(ChakError::CustomError("Empty file".to_string())), // Handle the case where the file is empty
    }
}
