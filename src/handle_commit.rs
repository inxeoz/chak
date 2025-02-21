use std::fs::File;
use serde::{Deserialize, Serialize};
use crate::config::{commits_fold, get_commit_log_file, get_stage_file, trees_fold};
use crate::handle_common::save_entity;
use crate::handle_tree::{clear_stage, TreeHashPointer, TreeObject};
use crate::hashing::{get_latest_pointer_line_from_file, hash_from_content, HashPointer, HashPointerTraits};
use crate::impl_hash_pointer_traits;
use crate::util::{deserialize_file_content, save_or_create_file, serialize_struct};

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub author: String,
    pub message: String,
    pub root_tree_pointer: TreeHashPointer,
}
//these custom hash pointer would have other field in future
#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct CommitHashPointer {
    fold_name: String,
    file_name: String,
}

impl_hash_pointer_traits!(CommitHashPointer);
impl CommitHashPointer {
    pub fn save_commit(commit: &Commit) -> Self {
        save_entity::<Self, Commit>(commit, &commits_fold())
    }

    pub fn load_commit(self) -> Commit {
        let deserialized_content = deserialize_file_content::<Commit>(&commits_fold().join(self.get_path()) ).expect("Failed to load commit file");
        deserialized_content
    }
}

pub fn create_commit(
    msg: String,
    author: Option<String>,
    root_tree_pointer: TreeHashPointer,
) -> Commit {
    Commit {
        message: msg,
        root_tree_pointer,
        author: author.unwrap_or("unknown".to_string()),
    }
}

pub fn append_commit_hash_pointer_to_commit_log_file(commit_hash_pointer: CommitHashPointer ) {
    save_or_create_file(
        &get_commit_log_file(), Some(&commit_hash_pointer.get_one_hash()), true, Some("\n")
    ).expect("cant save commit to commit log");
}

pub fn command_commit(m:String) {
    let staging_file =File::open(&get_stage_file()).expect("failed to open stage file");
    if let Some(latest_tree_pointer) = get_latest_pointer_line_from_file(&staging_file, false) {
        let commit_pointer = CommitHashPointer::save_commit(create_commit(
            m,
            Some("inxeoz".to_string()),
            latest_tree_pointer,
        ));

        append_commit_hash_pointer_to_commit_log_file(commit_pointer);

        clear_stage(); //we can clear stage
    } else {
        println!("No commit configured");
    }
}