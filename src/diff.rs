
use std::path::Path;
use std::{fs, io};
use std::fs::File;
use crate::diff_algo::{compare_hashed_content, to_hashed_content, HashedContent};
use crate::hashing::{ HashPointer};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Version {
    version_number: u32,
    version_type: VersionType,
    diff_from: u32,
    hash_pointer: HashPointer,
}
#[derive(Serialize, Deserialize, Debug)]
enum VersionType {
    DIFF,
    FILE,
}

pub fn deserialize_file_content<T: DeserializeOwned>(path: &Path) -> Result<T, io::Error> {
    let content_string = fs::read_to_string(path)?; // Reads file, propagates error if any
    let content = serde_json::from_str(&content_string)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?; // Converts serde error into io::Error
    Ok(content)
}

pub fn serialize_struct<T: Serialize>(data: &T) -> String {
    let serialized = serde_json::to_string_pretty(&data).expect("Failed to serialize");
    println!("{}", serialized);
    serialized
}

pub fn get_diff(prev_file: &File, new_file: &File) ->HashedContent {
    let first = to_hashed_content(&prev_file);
    let second = to_hashed_content(&new_file);
    let diff = compare_hashed_content(&first, &second);
    diff

}
