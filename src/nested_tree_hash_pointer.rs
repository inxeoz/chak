use crate::config::{commit_log_file_path, commits_fold, get_commit_log_file, get_project_dir, get_stage_file, nested_trees_fold, stage_file_path, trees_fold, versions_fold};


use crate::commit_hash_pointer::{Commit, CommitHashPointer};
use crate::common::{load_entity, save_entity};
use crate::hash_pointer::{HashPointer, HashPointerOwn, HashPointerTraits};

use crate::custom_error::ChakError;
use crate::impl_hash_pointer_common_traits;
use crate::tree_object::TreeObject;
use crate::util::{deserialize_file_content, file_to_lines, save_or_create_file, serialize_struct};
use clap::error::ErrorKind;
use std::cmp::Ordering;
use std::io;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct NestedTreeHashPointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_common_traits!(NestedTreeHashPointer);

impl HashPointerOwn for NestedTreeHashPointer {
    type Output = NestedTreeHashPointer;
    fn own<T: HashPointerTraits>(hash_pointer: &T) -> Result<Self::Output, ChakError> {
        if nested_trees_fold().join(hash_pointer.get_path()).exists() {
            Ok(NestedTreeHashPointer::_own(hash_pointer))
        } else {
            Err(ChakError::CustomError(format!(
                "{}",
                "tree hash pointer not found"
            )))
        }
    }
}
impl NestedTreeHashPointer {
    pub fn save_tree(tree: &mut TreeObject) -> NestedTreeHashPointer {
        tree.sort_children();
        Self::_own(&save_entity::<TreeObject>(tree, &nested_trees_fold()))
    }
    pub fn load_tree(&self) -> TreeObject {
        load_entity::<Self, TreeObject>(self, &nested_trees_fold())
    }

}
