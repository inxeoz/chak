
use serde::{Deserialize, Serialize};
use crate::config::{commit_log_file_path, commits_fold, get_commit_log_file, stage_file_path};

use std::path::PathBuf;
use crate::object::ObjectTraits;
use crate::root_tree_pointer::RootTreePointer;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitObject {
    pub author: String,
    pub message: String,
    pub root_tree_pointer: RootTreePointer,
}

impl ObjectTraits for CommitObject {
    fn containing_folder() -> PathBuf {
        commits_fold()
    }
}