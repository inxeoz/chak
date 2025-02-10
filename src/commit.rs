use crate::config::{commits_fold, history_fold, staging_area_fold};
use crate::diff::serialize_struct;
use crate::hashing::{get_latest_pointer_from_file, hash_from_content, hash_from_save_content, HashPointer};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub author: String,
    pub message: String,
    pub root_tree_pointer: HashPointer,
}

pub fn create_commit(
    msg: String,
    author: Option<String>,
    root_tree_pointer: HashPointer,
) -> Commit {
     Commit {
        message: msg,
        root_tree_pointer,
        author: author.unwrap_or("unknown".to_string()),
    }
}
pub fn save_commit(commit: Commit) -> io::Result<HashPointer> {
    let serialized_commit = serialize_struct(&commit);
    hash_from_save_content( &serialized_commit, &commits_fold())
}

pub fn attach_latest_root_pointer_to_stage(root_pointer: HashPointer) {
    let stage_file_path = &staging_area_fold().join("stage");
    let mut file = OpenOptions::new()
        .create(true) // Create the file if it doesn't exist
        .write(true) // Enable writing
        .truncate(true) // Clear file contents before writing
        .open(&stage_file_path)
        .expect("Couldn't open file");

    file.write_all(root_pointer.get_one_hash().as_ref())
        .expect("failed to attach root pointer to stage"); // Writing data
}

pub fn clear_commit_stage() {
    let stage_file_path = &staging_area_fold().join("stage");
    std::fs::write(stage_file_path, "").expect("Couldn't write to stage file");
}
