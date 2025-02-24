use crate::config::{
    commit_log_file_path, commits_fold, get_commit_log_file, get_project_dir, get_stage_file,
    stage_file_path, trees_fold, versions_fold,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Write;

use crate::commit::{Commit, CommitHashPointer};
use crate::common::{load_entity, save_entity};
use crate::hash_pointer::{HashPointer, HashPointerTraits};

use crate::custom_error::ChakError;
use crate::impl_hash_pointer_traits;
use crate::tree_object::TreeObject;
use crate::util::{deserialize_file_content, save_or_create_file, serialize_struct};
use std::cmp::Ordering;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct TreeHashPointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_traits!(TreeHashPointer);
impl TreeHashPointer {
    fn own(hash_pointer: &HashPointer) -> TreeHashPointer {
        TreeHashPointer {
            fold_name: hash_pointer.get_fold_name(),
            file_name: hash_pointer.get_file_name(),
        }
    }
    pub fn save_tree(tree: &mut TreeObject) -> TreeHashPointer {
        tree.sort_children();
        Self::own(&save_entity::<TreeObject>(tree, &trees_fold()))
    }
    pub fn load_tree(&self) -> TreeObject {
        load_entity::<Self, TreeObject>(self, &trees_fold())
    }

    pub fn attach_tree_to_stage(&self) {
        save_or_create_file(&stage_file_path(), Some(&self.get_one_hash()), true, None)
            .expect("failed to attach root pointer to stage");
    }
    pub fn get_latest_tree_root_pointer(
        from_commit_log: bool,
    ) -> Result<TreeHashPointer, ChakError> {
        if from_commit_log && commit_log_file_path().exists() {
            match CommitHashPointer::get_latest_commit_hash_pointer() {
                Ok(latest_commit_pointer) => {
                    let latest_commit = latest_commit_pointer.load_commit();
                    Ok(latest_commit.root_tree_pointer)
                }
                Err(e) => Err(e.into()),
            }
        } else {
            match HashPointer::get_latest_pointer_line_from_file(&get_stage_file()?, true) {
                Ok(latest_tree_pointer) => Ok(Self::own(&latest_tree_pointer)),
                Err(e) => Err(e.into()),
            }
        }
    }
}
