use std::fs::File;
use serde::{Deserialize, Serialize};
use crate::config::{commit_log_file_path, commits_fold, get_commit_log_file, root_trees_fold, stage_file_path};
use crate::common::{load_entity, save_entity};
use crate::root_tree_pointer::{ RootTreeHashPointer};
use crate::impl_hash_pointer_common_traits;
use crate::util::{save_or_create_file};
use std::path::PathBuf;
use std::cmp::Ordering;
use crate::commit_object::Commit;
use crate::custom_error::ChakError;
use crate::hash_pointer::{HashPointer, HashPointerCommonTraits, HashPointerCoreTraits};


//these custom hash pointer would have other field in future
#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct CommitHashPointer {
    fold_name: String,
    file_name: String,
}

impl_hash_pointer_common_traits!(CommitHashPointer);

impl HashPointerCoreTraits for CommitHashPointer {
    type Output = CommitHashPointer;

    fn own<T: HashPointerCommonTraits>(hash_pointer: &T) -> Result<Self::Output, ChakError> {
        if Self::verify_existing(hash_pointer) {
            Ok(CommitHashPointer {
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
        commits_fold().join(hash_pointer.get_path()).exists()
    }

}
impl CommitHashPointer {

    pub fn save_commit(commit: &Commit) -> Result<CommitHashPointer, ChakError> {
        Self::own(&save_entity(commit))
    }

    pub fn load_commit(&self) -> Commit {
        load_entity::<Self, Commit>(self, &commits_fold())
    }

    pub fn get_latest_commit_hash_pointer() -> Result<CommitHashPointer, ChakError> {
        // as commit log file created at initialization
        let commit_file = get_commit_log_file()?;
        Ok( HashPointer::get_latest_pointer_line_from_file::<CommitHashPointer>(&commit_file, true)?)
    }
}

pub fn create_commit(
    msg: String,
    author: Option<String>,
    root_tree_pointer: RootTreeHashPointer,
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

pub fn command_commit(m:String) -> Result<(), ChakError> {

    if let Ok(all_tree_pointers) = RootTreeHashPointer::get_pointers_from_stage(){

        for (index, tree_pointer) in all_tree_pointers.iter().rev().enumerate(){
            if index == 0 {
                let commit_pointer = CommitHashPointer::save_commit(&create_commit(
                    m.clone(),
                    Some("inxeoz".to_string()),
                    tree_pointer.to_owned(),
                ))?;

                append_commit_hash_pointer_to_commit_log_file(commit_pointer);
                return Ok(());
            }
        }
        std::fs::write(stage_file_path(), "").map_err(
            |_| ChakError::CustomError("Failed to save commit to commit log".to_string())
        )
    } else {
        Err(ChakError::CustomError("No stage file".to_string()))
    }
}