use crate::config_global::MIN_HASH_LENGTH;
use crate::custom_error::ChakError;
use crate::hash_pointer::{HashPointer, HashPointerTraits};
use crate::util::file_to_lines;
use crate::util::{
    deserialize_file_content, file_to_string, save_or_create_file, serialize_struct,
};
use serde::de::DeserializeOwned;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};

impl HashPointer {
    fn _from_hash_string(hash: String) -> Self {
        Self::new(hash[..2].to_string(), hash[2..].to_string())
    }

    pub fn from_hash_pointer_string(hash: String) -> Result<Self, ChakError> {
        if hash.len() < MIN_HASH_LENGTH {
            return Err(ChakError::CustomError(
                "Invalid hash string length".to_string(),
            ));
        }
        Ok(Self::_from_hash_string(hash))
    }

    pub fn combine(first: &Self, second: &Self) -> Self {
        Self::from_string(&(first.get_one_hash() + &second.get_one_hash()))
    }

    pub fn from_file_path(file_path: &Path) -> io::Result<Self> {
        let mut file = File::open(file_path)?;
        Ok(Self::from_file(&mut file))
    }

    pub fn from_file(file: &File) -> Self {
        let mut buf_file = BufReader::new(file);
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 1024];

        while let Ok(bytes_read) = buf_file.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }
        Self::_from_hash_string(format!("{:x}", hasher.finalize()))
    }

    pub fn from_save_string(content: &str, save_dir: &Path) -> io::Result<Self> {
        let hash_pointer = Self::from_string(content);
        save_or_create_file(
            &save_dir.join(hash_pointer.get_path()),
            Some(content),
            false,
            None,
        )?;
        Ok(hash_pointer)
    }

    pub fn from_pointers<T: HashPointerTraits>(pointers: Vec<T>) -> Result<Self, ChakError> {
        if pointers.is_empty() {
            return Err(ChakError::CustomError(
                "Empty hash pointer vector".to_string(),
            ));
        }
        let mut hasher = Sha256::new();
        for pointer in pointers {
            hasher.update(pointer.get_one_hash().as_bytes());
        }
        Ok(Self::_from_hash_string(format!("{:x}", hasher.finalize())))
    }

    pub fn from_string(content: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        Self::_from_hash_string(format!("{:x}", hasher.finalize()))
    }

    // Rest of your functions remain unchanged...

    pub fn from_string_vec(strings: &[String]) -> Self {
        let mut hasher = Sha256::new();
        for string in strings {
            hasher.update(string.as_bytes());
        }
        Self::_from_hash_string(format!("{:x}", hasher.finalize()))
    }

    // pub fn hash_from_save_tree<T: SelfTraits>(
    //     save_dir: &Path,
    //     tree_object: &mut TreeObject,
    // ) -> io::Result<T> {
    //     //sort the object so that it always produce same hash for same content or object no matter what their position
    //     tree_object.sort_children();
    //     let content = serialize_struct(&tree_object);
    //     hash_from_save_content(&content, save_dir)
    // }

    pub fn get_latest_pointer_line_from_file(
        file: &File,
        from_bottom: bool,
    ) -> Result<Self, ChakError> {
        let lines = file_to_lines(file);

        let line = if from_bottom {
            lines.last() // Get the last line
        } else {
            lines.first() // Get the first line
        };

        if let Some(line) = line {
            Self::from_hash_pointer_string(line.to_string())
        } else {
            Err(ChakError::CustomError(
                "hash pointer line not found in file".to_string(),
            ))
        }
    }

    pub fn and_string_from_file_path_ref(file_path: &Path) -> io::Result<(Self, String)> {
        let mut file = File::open(file_path)?;
        let content = file_to_string(&mut file)?;
        let hash_pointer = Self::from_string(&content);
        Ok((hash_pointer, content))
    }
}
