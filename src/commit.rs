use std::fs::File;
use serde::{Deserialize, Serialize};
use crate::config::{commit_log_file_path, commits_fold, get_commit_log_file, get_stage_file, trees_fold};
use crate::common::{load_entity, save_entity};
use crate::tree_hash_pointer::{clear_stage, TreeHashPointer};
use crate::impl_hash_pointer_traits;
use crate::util::{deserialize_file_content, save_or_create_file, serialize_struct};
use std::path::PathBuf;
use std::cmp::Ordering;
use crate::custom_error::ChakError;
use crate::hash_pointer::{HashPointer, HashPointerTraits};


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

    fn own(hash_pointer: HashPointer) -> CommitHashPointer {
        CommitHashPointer {
            fold_name: hash_pointer.get_fold_name(),
            file_name: hash_pointer.get_file_name(),
        }
    }
    pub fn save_commit(commit: &Commit) -> Self {
        Self::own(save_entity::<Commit>(commit, &commits_fold()))
    }

    pub fn load_commit(&self) -> Commit {
        load_entity::<Self, Commit>(self, &commits_fold())
    }

    pub fn get_latest_commit_hash_pointer() -> Result<CommitHashPointer, ChakError> {
        // as commit log file created at initialization
        let commit_file = get_commit_log_file()?;
        match HashPointer::get_latest_pointer_line_from_file(&commit_file, true) {
            Ok(hash_pointer) => {
                Ok(CommitHashPointer::own(hash_pointer))
            }
            Err(e) => {
                Err(e.into())
            }
        }
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
        &commit_log_file_path(), Some(&commit_hash_pointer.get_one_hash()), true, Some("\n")
    ).expect("cant save commit to commit log");
}

pub fn command_commit(m:String) {

    if let Ok(latest_tree_pointer) = TreeHashPointer::get_latest_tree_root_pointer(true){
        let commit_pointer = CommitHashPointer::save_commit(&create_commit(
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