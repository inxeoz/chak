
use serde::{Deserialize, Serialize};
use crate::config::{commit_log_file_path, commits_fold, get_commit_log_file, stage_file_path};
use crate::root_tree_pointer::{ RootTreeHashPointer};
use std::path::PathBuf;
use crate::object::ObjectTraits;

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub author: String,
    pub message: String,
    pub root_tree_pointer: RootTreeHashPointer,
}

impl ObjectTraits for Commit {
    fn containing_folder(&self) -> PathBuf {
        commits_fold()
    }
}