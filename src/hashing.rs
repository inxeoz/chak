use crate::config::MIN_HASH_LENGTH;
use crate::macros::create_file;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use crate::tree_object::TreeObject;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Eq)]
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
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other)) // Delegate to Ord
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
        let has_p =hash_combine(&self, &hash_from_content(content));
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


pub fn hash_pointer_from_hash_string(hash: String) -> HashPointer {
    if hash.len() < MIN_HASH_LENGTH {
        panic!("Invalid hash length");
    }
    HashPointer {
        fold_name: hash[..2].to_string(),
        file_name: hash[2..].to_string(),
    }
}

pub fn hash_combine(first: &HashPointer, second: &HashPointer) -> HashPointer {
    hash_from_content(
        first.get_one_hash() + &second.get_one_hash(),
    )
}

pub fn hash_from_file(path: &Path) -> HashPointer {
        let mut file = BufReader::new(File::open(path).expect("Failed to open file"));
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 1024];

        while let Ok(bytes_read) = file.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }

        hash_pointer_from_hash_string(format!("{:x}", hasher.finalize()))
    }

    pub fn hash_from_save_blob(file_path: &Path, save_dir: &Path) -> HashPointer {
        let hash_pointer = hash_from_file(file_path);
        let blob_file = save_dir.join(hash_pointer.get_path());
        if blob_file.exists() {
            println!("file already exists");
            return hash_pointer;
        }
        println!("file ---> name {}", file_path.display());
        let content = fs::read(file_path).expect("Failed to read file");
        create_file(save_dir, &hash_pointer, Some(content));
        hash_pointer
    }
    pub fn hash_from_save_content(save_dir: &Path, content: String) -> HashPointer {
        let hash_pointer = hash_from_content(content.clone());

        if !save_dir.join(hash_pointer.get_path()).exists() {
            create_file(save_dir, &hash_pointer, Some(content.into_bytes()));
        } else {
            println!(
                "Blob already exists: {}",
                hash_pointer.get_path().display()
            );
        }

        hash_pointer
    }

    pub fn hash_from_pointers(pointers: Vec<HashPointer>) -> HashPointer {
        if pointers.is_empty() {
            panic!("Empty hash pointer vector");
        }

        let mut hasher = Sha256::new();
        for pointer in pointers {
            hasher.update(pointer.get_one_hash().as_bytes());
        }

        hash_pointer_from_hash_string(format!("{:x}", hasher.finalize()))
    }

    pub fn hash_from_content(content: String) -> HashPointer {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        hash_pointer_from_hash_string(format!("{:x}", hasher.finalize()))
    }

    pub fn hash_from_string_vec(strings: &[String]) -> HashPointer {
        let mut hasher = Sha256::new();
        for string in strings {
            hasher.update(string.as_bytes());
        }
        hash_pointer_from_hash_string(format!("{:x}", hasher.finalize()))
    }

pub fn hash_from_save_tree(save_dir: &Path,  tree_object: &mut TreeObject) -> HashPointer {
    tree_object.sort_children();
    let content = serde_json::to_string(&tree_object).expect("Failed to serialize tree_object");
    hash_from_save_content(save_dir, content)
}

