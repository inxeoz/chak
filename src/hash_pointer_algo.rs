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
use crate::hash_pointer::{HashPointer, HashPointerTraits};

pub fn _hash_pointer_from_hash_string(hash: String) -> HashPointer {
    HashPointer::new(
        hash[..2].to_string(),
       hash[2..].to_string(),
    )
}

pub fn hash_pointer_from_hash_string(hash: String) -> Result<HashPointer, ChakError> {
    if hash.len() < MIN_HASH_LENGTH {
        return Err(ChakError::CustomError(
            "Invalid hash string length".to_string(),
        ));
    }
    Ok(
        _hash_pointer_from_hash_string(hash)

    )
}

pub fn hash_combine<T: HashPointerTraits>(first: &T, second: &T) -> T {
    hash_from_content(&(first.get_one_hash() + &second.get_one_hash()))
}

pub fn hash_from_file(file: &File) -> HashPointer{
    let mut buf_file = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 1024];

    while let Ok(bytes_read) = buf_file.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    _hash_pointer_from_hash_string(format!("{:x}", hasher.finalize()))
}

pub fn hash_from_save_content(content: &str, save_dir: &Path) -> io::Result<HashPointer> {
    let hash_pointer = hash_from_content(content);
    save_or_create_file(
        &save_dir.join(hash_pointer.get_path()),
        Some(content),
        false,
        None,
    )?;
    Ok(hash_pointer)
}

pub fn hash_from_pointers<T: HashPointerTraits>(pointers: Vec<T>) -> Result<HashPointer, ChakError> {
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

pub fn hash_from_content(content: &str) -> HashPointer {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    _hash_pointer_from_hash_string(format!("{:x}", hasher.finalize()))
}


// Rest of your functions remain unchanged...

pub fn hash_from_string_vec(strings: &[String]) ->HashPointer {
    let mut hasher = Sha256::new();
    for string in strings {
        hasher.update(string.as_bytes());
    }
    _hash_pointer_from_hash_string(format!("{:x}", hasher.finalize()))
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

pub fn get_latest_pointer_line_from_file(file: &File, from_bottom: bool) -> Option<HashPointer> {
    let lines = file_to_lines(file);

    let line = if from_bottom {
        lines.last() // Get the last line
    } else {
        lines.first() // Get the first line
    };

    line.and_then(|l| hash_pointer_from_hash_string(l.to_string()).ok())
}



pub fn hash_and_content_from_file_path_ref(file_path: &Path) -> io::Result<(HashPointer, String)> {
    let mut file = File::open(file_path)?;
    let content = file_to_string(&mut file)?;
    let hash_pointer = hash_from_content(&content);
    Ok((hash_pointer, content))
}



