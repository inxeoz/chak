
use serde::{Deserialize, Serialize};
use crate::config::{get_commits_fold_path};

use std::path::PathBuf;
use crate::chak_traits::ObjectCommonTraits;
use crate::root_tree_pointer::RootTreePointer;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitObject {
    pub author: String,
    pub message: String,
    pub root_tree_pointer: RootTreePointer,
}

impl ObjectCommonTraits for CommitObject {
    fn containing_folder() -> PathBuf {
        get_commits_fold_path()
    }
}