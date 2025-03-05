use crate::config::{
    commit_log_file_path, commits_fold, get_commit_log_file, get_project_dir, get_stage_file,
    root_trees_fold, stage_file_path, versions_fold,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Write;

use crate::common::{load_entity, save_entity};
use crate::hash_pointer::{HashPointer, HashPointerCommonTraits, HashPointerCoreTraits};

use crate::commit_pointer::CommitHashPointer;
use crate::custom_error::ChakError;
use crate::impl_hash_pointer_common_traits;
use crate::root_tree_object::{NestedTreeObject, RootTreeObject};
use crate::util::save_or_create_file;
use std::cmp::Ordering;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct RootTreeHashPointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_common_traits!(RootTreeHashPointer);

impl HashPointerCoreTraits for RootTreeHashPointer {
    type Output = RootTreeHashPointer;
    fn own<T: HashPointerCommonTraits>(hash_pointer: &T) -> Result<Self::Output, ChakError> {
        if Self::verify_existing(hash_pointer) {
            Ok(RootTreeHashPointer {
                file_name: hash_pointer.get_file_name(),
                fold_name: hash_pointer.get_fold_name(),
            })
        } else {
            Err(ChakError::CustomError(format!(
                "{}",
                "tree hash pointer not found"
            )))
        }
    }

    fn verify_existing<T: HashPointerCommonTraits>(hash_pointer: &T) -> bool {
        root_trees_fold().join(hash_pointer.get_path()).exists()
    }
}
impl RootTreeHashPointer {
    pub fn save_tree(tree: &mut RootTreeObject) -> Result<RootTreeHashPointer, ChakError> {
        tree.sort_children();
        Self::own(&save_entity(tree))
    }
    pub fn load_tree(&self) -> RootTreeObject {
        load_entity::<Self, RootTreeObject>(self, &root_trees_fold())
    }

    pub fn attach_tree_to_stage(&self) {
        save_or_create_file(
            &stage_file_path(),
            Some(&self.get_one_hash()),
            true,
            Some("\n"),
        )
        .expect("failed to attach root pointer to stage");
    }

    pub fn get_latest_pointer_from_commit_log() -> Result<RootTreeHashPointer, ChakError> {
        if commit_log_file_path().exists() {
            Ok(CommitHashPointer::get_latest_commit_hash_pointer()?
                .load_commit()
                .root_tree_pointer)
        } else {
            Err(ChakError::CustomError(
                "commit_log_file path doesn't exist".to_string(),
            ))
        }
    }

    pub fn get_pointers_from_commit_log() -> Result<Vec<RootTreeHashPointer>, ChakError> {
        if commit_log_file_path().exists() {
            Ok(HashPointer::get_pointer_lines_from_file(
                &get_commit_log_file()?,
            )?)
        } else {
            Err(ChakError::CustomError(
                "commit_log_file path doesn't exist".to_string(),
            ))
        }
    }

    pub fn get_latest_pointer_from_stage() -> Result<RootTreeHashPointer, ChakError> {
        if stage_file_path().exists() {
            Ok(HashPointer::get_latest_pointer_line_from_file::<
                RootTreeHashPointer,
            >(&get_stage_file()?, true)?)
        } else {
            Err(ChakError::CustomError(
                "stage file path doesn't exist".to_string(),
            ))
        }
    }

    pub fn get_pointers_from_stage() -> Result<Vec<RootTreeHashPointer>, ChakError> {
        if stage_file_path().exists() {
            Ok(HashPointer::get_pointer_lines_from_file(&get_stage_file()?)?)
        } else {
            Err(ChakError::CustomError(
                "stage file path doesn't exist".to_string(),
            ))
        }
    }
}
