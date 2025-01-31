use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use crate::diff::{ serialize_struct};
use crate::hashing::{hash_from_save_content, hash_pointer_from_hash_string, HashPointer};
use crate::macros::append_to_file;
use serde::{Deserialize, Serialize};
use crate::config::{commit_history_fold, commits_fold, staging_area_fold};

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
     pub author: String,
    pub message: String,
    pub root_tree_pointer: HashPointer,
}
pub fn save_commit(commit: Commit) -> HashPointer{
    let serialized_commit = serialize_struct(&commit);
     hash_from_save_content(&commits_fold(), serialized_commit)
}


pub fn append_commit_pointer_to_history(commit_pointer: HashPointer) {
    append_to_file(
        &commit_history_fold().join("commit_log"),
        &commit_pointer.get_one_hash(),
    )
        .expect("Failed to perform commit");
}

pub fn create_commit_from_stage(msg: String, author: Option<String>) {
    let commit = Commit {
        message: msg,
        root_tree_pointer: get_root_pointer_from_stage(),
        author: author.unwrap_or("unknown".to_string()),
    };

    let commit_pointer = save_commit(commit);
    append_commit_pointer_to_history(commit_pointer);

}

pub fn get_root_pointer_from_stage() -> HashPointer {
    let file = File::open(staging_area_fold().join("stage")).expect("Failed to open stage");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<io::Result<_>>().expect("Failed to read stage file");
    let hps = lines.first().expect("Nothing to stage it!").clone();
    println!("root pointer: {:?}", hps);
    hash_pointer_from_hash_string(hps)
}
