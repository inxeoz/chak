use crate::config::{blob_fold};
use crate::macros::create_file;
use std::{fs, io};
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use crate::diff_algo::{compare_hashed_content, to_interconnected_line};
use crate::hashing::{hash_from_content, hash_from_file, hash_from_save_blob, HashPointer};

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
// pub fn start_versioning(file_path: &Path) {
//     let version_blob_pointer = hash_from_file(file_path);
//     let version = version_fold().join(version_blob_pointer.get_path());
//     if !&version.exists() && file_path.is_file() {
//         let file_blob_pointer = hash_from_save_blob(file_path, &blob_fold());
//         let new_version = Version {
//             version_number: 0,
//             version_type: VersionType::FILE,
//             diff_from: 0,
//             hash_pointer: file_blob_pointer,
//         };
//         let new_version_vec = Vec::from([new_version]);
//         let serialized_version = serialize_struct(&new_version_vec);
//         create_file(
//             &version_fold(),
//             &version_blob_pointer,
//             Some(serialized_version.into_bytes()),
//         );
//     } else {
//         let deserialized_version: Vec<Version> = deserialize_file_content(&version).expect("TODO");
//         let latest_version = deserialized_version.last().expect("No version found");
//         if latest_version.version_number > 0 {
//             if hash_from_file(file_path).get_one_hash() != latest_version.hash_pointer.get_one_hash()
//             {
//             }
//         }
//     }
// }


pub fn deserialize_file_content<T: DeserializeOwned>(path: &Path) -> Result<T, io::Error> {
    let content_string = fs::read_to_string(path)?;  // Reads file, propagates error if any
    let content = serde_json::from_str(&content_string)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?; // Converts serde error into io::Error
    Ok(content)
}

pub fn serialize_struct<T: Serialize>(data: &T) -> String {
    let serialized = serde_json::to_string(&data).expect("Failed to serialize");
    serialized
}

pub fn get_diff(prev_file: &Path, new_file: &Path) {

    // Generate mappings
    let first = to_interconnected_line(prev_file);
    let second = to_interconnected_line(new_file);

    // Serialize and print mappings
    println!("Line to Hash:");
    println!("{}", serde_json::to_string_pretty(&first.line_to_hash).unwrap());

    println!("Hash to Content:");
    println!("{}", serde_json::to_string_pretty(&first.hash_to_content).unwrap());

    // Serialize and print mappings
    println!("Line to Hash:");
    println!("{}", serde_json::to_string_pretty(&second.line_to_hash).unwrap());

    println!("Hash to Content:");
    println!("{}", serde_json::to_string_pretty(&second.hash_to_content).unwrap());

    let diff = compare_hashed_content(first, second);

    // // Serialize and print mappings
    // println!("Line to Hash:");
    // println!("{}", serde_json::to_string_pretty(&new.line_to_hash).unwrap());
    //
    // println!("Hash to Content:");
    // println!("{}", serde_json::to_string_pretty(&new.hash_to_content).unwrap());



}