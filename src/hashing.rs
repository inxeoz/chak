use crate::config::MIN_HASH_LENGTH;
use crate::macros::create_file;
use clap::builder::Str;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::ops::Add;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HashPointer {
    fold_name: String,
    file_name: String,
}

impl PartialEq for HashPointer {
    fn eq(&self, other: &Self) -> bool {
        self.get_one_hash() == other.get_one_hash()
    }
}

impl HashPointer {

    pub fn replace(&mut self, hash_pointer: &HashPointer) {
        self.fold_name = hash_pointer.get_fold_name();
        self.file_name = hash_pointer.get_file_name();

    }

    fn from_hash_string(hash: String) -> Self {
        if hash.len() < MIN_HASH_LENGTH {
            panic!("Invalid hash length");
        }
        Self {
            fold_name: hash[..2].to_string(),
            file_name: hash[2..].to_string(),
        }
    }

    pub fn combine(first: &Self, second: &Self) -> Self {
        Self::from_content(
            (first.get_one_hash() + &second.get_one_hash()),
        )
    }

    pub fn update_hash(&mut self, content: String) {
        let has_p = Self::combine(&self, &Self::from_content(content));
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

    pub fn from_file(path: &Path) -> Self {
        let mut file = BufReader::new(File::open(path).expect("Failed to open file"));
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 1024];

        while let Ok(bytes_read) = file.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }

        Self::from_hash_string(format!("{:x}", hasher.finalize()))
    }

    pub fn save_blob(file_path: &Path, save_dir: &Path) -> Self {
        let hash_pointer = Self::from_file(file_path);
        let content = fs::read_to_string(file_path).expect("Failed to read file content");
        create_file(save_dir, &hash_pointer, Some(content));
        hash_pointer
    }

    pub fn save_blob_from_content(save_dir: &Path, content: String) -> Self {
        let hash_pointer = Self::from_content(content.clone());

        if !save_dir.join(hash_pointer.get_path()).exists() {
            create_file(save_dir, &hash_pointer, Some(content));
        } else {
            println!(
                "Blob already exists: {}",
                hash_pointer.get_path().display()
            );
        }

        hash_pointer
    }

    pub fn hash_from_pointers(pointers: Vec<Self>) -> Self {
        if pointers.is_empty() {
            panic!("Empty hash pointer vector");
        }

        let mut hasher = Sha256::new();
        for pointer in pointers {
            hasher.update(pointer.get_one_hash().as_bytes());
        }

        Self::from_hash_string(format!("{:x}", hasher.finalize()))
    }

    pub fn from_path(path: &Path) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(
            path.to_str()
                .expect("Failed to convert path to string")
                .as_bytes(),
        );
        Self::from_hash_string(format!("{:x}", hasher.finalize()))
    }

    pub fn from_content(content: String) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        Self::from_hash_string(format!("{:x}", hasher.finalize()))
    }

    pub fn from_hash_string_vec(strings: &[String]) -> Self {
        let mut hasher = Sha256::new();
        for string in strings {
            hasher.update(string.as_bytes());
        }
        Self::from_hash_string(format!("{:x}", hasher.finalize()))
    }
}
