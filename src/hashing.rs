use crate::custom_error::ChakError;
use crate::global_config::MIN_HASH_LENGTH;
use crate::util::{file_to_string, save_or_create_file, serialize_struct};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::cmp::Ordering;
use std::io;
use crate::diff_algo::file_to_lines;
use crate::handle_tree::TreeObject;

pub trait HashPointerTraits {
    fn new(fold_name: String, file_name: String) -> Self; // Added for construction
    fn replace(&mut self, pointer: &Self);
    fn update_hash(&mut self, content: String);
    fn get_fold_name(&self) -> String;
    fn get_file_name(&self) -> String;
    fn get_one_hash(&self) -> String;
    fn get_path(&self) -> PathBuf;
    fn set_fold_name(&mut self, fold_name: String);
    fn set_file_name(&mut self, file_name: String);
}


#[macro_export]
macro_rules! impl_hash_pointer_traits {
    ($t:ty) => {
        impl PartialEq for $t {
            fn eq(&self, other: &Self) -> bool {
                self.get_one_hash() == other.get_one_hash()
            }
        }

        impl PartialOrd for $t {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $t {
            fn cmp(&self, other: &Self) -> Ordering {
                self.get_fold_name()
                    .cmp(&other.get_fold_name())
                    .then_with(|| self.get_file_name().cmp(&other.get_file_name()))
            }
        }

        impl HashPointerTraits for $t {
            
             fn new(fold_name: String, file_name: String) -> Self {
                Self { fold_name, file_name }
            }
            fn replace(&mut self, pointer: &Self) {
                self.set_fold_name(pointer.get_fold_name());
                self.set_file_name(pointer.get_file_name());
            }

            fn update_hash(&mut self, content: String) {
                let has_p = Self::new(self.get_fold_name(), hash_from_content::<Self>(&content).get_file_name());
                self.replace(&has_p);
            }

            fn get_fold_name(&self) -> String {
                self.fold_name.clone()
            }

            fn get_file_name(&self) -> String {
                self.file_name.clone()
            }

            fn get_one_hash(&self) -> String {
                self.fold_name.clone() + &self.file_name
            }

            fn get_path(&self) -> PathBuf {
                PathBuf::from(&self.fold_name).join(&self.file_name)
            }

            fn set_fold_name(&mut self, fold_name: String) {
                self.fold_name = fold_name;
            }

            fn set_file_name(&mut self, file_name: String) {
                self.file_name = file_name;
            }

            fn new(fold_name: String, file_name: String) -> Self {
                Self { fold_name, file_name }
            }
        }
    };
}


#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct HashPointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_traits!(HashPointer);

pub fn _hash_pointer_from_hash_string<T: HashPointerTraits>(hash: String) -> T {
    T::new(
        hash[..2].to_string(),
       hash[2..].to_string(),
    )
}

pub fn hash_pointer_from_hash_string<T: HashPointerTraits>(hash: String) -> Result<T, ChakError> {
    if hash.len() < MIN_HASH_LENGTH {
        return Err(ChakError::CustomError(
            "Invalid hash string length".to_string(),
        ));
    }
    Ok(_hash_pointer_from_hash_string::<T>(hash))
}

pub fn hash_combine<T: HashPointerTraits>(first: &T, second: &T) -> T {
    hash_from_content(&(first.get_one_hash() + &second.get_one_hash()))
}

pub fn hash_from_file<T: HashPointerTraits>(file: &File) -> T {
    let mut buf_file = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 1024];

    while let Ok(bytes_read) = buf_file.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    _hash_pointer_from_hash_string::<T>(format!("{:x}", hasher.finalize()))
}

pub fn hash_from_save_content<T: HashPointerTraits>(content: &str, save_dir: &Path) -> io::Result<T> {
    let hash_pointer = hash_from_content::<T>(content);
    save_or_create_file(
        &save_dir.join(hash_pointer.get_path()),
        Some(content),
        false,
        None,
    )?;
    Ok(hash_pointer)
}

pub fn hash_from_pointers<T: HashPointerTraits>(pointers: Vec<T>) -> Result<T, ChakError> {
    if pointers.is_empty() {
        return Err(ChakError::CustomError(
            "Empty hash pointer vector".to_string(),
        ));
    }
    let mut hasher = Sha256::new();
    for pointer in pointers {
        hasher.update(pointer.get_one_hash().as_bytes());
    }
    Ok(_hash_pointer_from_hash_string::<T>(format!(
        "{:x}",
        hasher.finalize()
    )))
}

pub fn hash_from_content<T: HashPointerTraits>(content: &str) -> T {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    _hash_pointer_from_hash_string::<T>(format!("{:x}", hasher.finalize()))
}


// Rest of your functions remain unchanged...

pub fn hash_from_string_vec<T: HashPointerTraits>(strings: &[String]) -> T {
    let mut hasher = Sha256::new();
    for string in strings {
        hasher.update(string.as_bytes());
    }
    _hash_pointer_from_hash_string::<T>(format!("{:x}", hasher.finalize()))
}

// pub fn hash_from_save_tree<T: HashPointerTraits>(
//     save_dir: &Path,
//     tree_object: &mut TreeObject,
// ) -> io::Result<T> {
//     //sort the object so that it always produce same hash for same content or object no matter what their position
//     tree_object.sort_children();
//     let content = serialize_struct(&tree_object);
//     hash_from_save_content(&content, save_dir)
// }

pub fn get_latest_pointer_line_from_file<T: HashPointerTraits>(file: &File, from_bottom: bool) -> Option<T> {
    let lines = file_to_lines(file);

    let line = if from_bottom {
        lines.last() // Get the last line
    } else {
        lines.first() // Get the first line
    };

    line.and_then(|l| hash_pointer_from_hash_string(l.to_string()).ok())
}



pub fn hash_and_content_from_file_path_ref<T: HashPointerTraits>(file_path: &Path) -> io::Result<(T, String)> {
    let mut file = File::open(file_path)?;
    let content = file_to_string(&mut file)?;
    let hash_pointer = hash_from_content(&content);
    Ok((hash_pointer, content))
}



