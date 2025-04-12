use crate::restricted;
use crate::chak_traits::HashPointerTraits;
use crate::config::{
    get_commit_log_file_path, get_commit_log_file, get_stage_file,
    get_root_trees_fold_path, get_stage_file_path,
};
use serde::{Deserialize, Serialize};
use crate::common::{load_entity, save_entity};
use crate::hash_pointer::{HashPointer};

use crate::custom_error::ChakError;
use crate::impl_hash_pointer_common_traits;
use crate::root_tree_object::{ RootTreeObject};
use crate::util::save_or_create_file;
use std::cmp::Ordering;
use crate::chak_traits::ChakPointerTraits;
use crate::commit_pointer::CommitPointer;

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct RootTreePointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_common_traits!(RootTreePointer, RootTreeObject);

impl RootTreePointer {
    pub fn save_tree(tree: &mut RootTreeObject) -> Result<RootTreePointer, ChakError> {
        tree.sort_children();
        Self::own(&save_entity(tree)?)
    }
    pub fn load_tree(&self) -> RootTreeObject {
        load_entity::<Self, RootTreeObject>(self, &get_root_trees_fold_path())
    }

    pub fn attach_tree_to_stage(&self) {
        save_or_create_file(
            &get_stage_file_path(),
            Some(&self.get_one_hash()),
            true,
            Some("\n"),
        )
        .expect("failed to attach root pointer to stage");
    }

    pub fn get_latest_pointer_from_commit_log() -> Result<RootTreePointer, ChakError> {
        if get_commit_log_file_path().exists() {
            Ok(CommitPointer::get_latest_commit_hash_pointer()?
                .load_commit()
                .root_tree_pointer)
        } else {
            Err(ChakError::CustomError(
                "commit_log_file path doesn't exist".to_string(),
            ))
        }
    }

    pub fn get_pointers_from_commit_log() -> Result<Vec<RootTreePointer>, ChakError> {
        if get_commit_log_file_path().exists() {
            Ok(HashPointer::get_pointer_lines_from_file(
                &get_commit_log_file()?,
            )?)
        } else {
            Err(ChakError::CustomError(
                "commit_log_file path doesn't exist".to_string(),
            ))
        }
    }

    pub fn get_latest_pointer_from_stage() -> Result<RootTreePointer, ChakError> {
        if get_stage_file_path().exists() {
            Ok(HashPointer::get_latest_pointer_line_from_file::<
                RootTreePointer,
            >(&get_stage_file()?, true)?)
        } else {
            Err(ChakError::CustomError(
                "stage file path doesn't exist".to_string(),
            ))
        }
    }

    pub fn get_pointers_from_stage() -> Result<Vec<RootTreePointer>, ChakError> {
        if get_stage_file_path().exists() {
            Ok(HashPointer::get_pointer_lines_from_file(&get_stage_file()?)?)
        } else {
            Err(ChakError::CustomError(
                "stage file path doesn't exist".to_string(),
            ))
        }
    }
}
