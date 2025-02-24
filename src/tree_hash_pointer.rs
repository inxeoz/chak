use crate::config::{commits_fold, get_commit_log_file, get_project_dir, get_stage_file, stage_file_path, trees_fold, versions_fold};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Write;

use crate::commit::{Commit, CommitHashPointer};
use crate::common::{load_entity, save_entity};
use crate::hash_pointer::{HashPointer, HashPointerTraits};

use std::path::PathBuf;
use std::cmp::Ordering;
use crate::custom_error::ChakError;
use crate::impl_hash_pointer_traits;
use crate::tree_object::TreeObject;
use crate::util::{deserialize_file_content, save_or_create_file, serialize_struct};

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

    pub fn get_latest_tree_root_pointer(from_stage: bool) -> Result<TreeHashPointer, ChakError> {
        if from_stage && stage_file_path().exists() {
            match HashPointer::get_latest_pointer_line_from_file(&get_stage_file()?, true) {
                Ok(latest_tree_pointer) => {
                   Ok(Self::own(&latest_tree_pointer))
                }
                Err(e) => {
                    Err(e.into())
                }
            }
        } else {
            match CommitHashPointer::get_latest_commit_hash_pointer() {
                Ok(latest_commit_pointer) => {
                    let latest_commit = latest_commit_pointer.load_commit();
                    Ok(latest_commit.root_tree_pointer)
                }
                Err(e) => {
                    Err(e.into())
                }
            }

        }
    }
}
pub fn attach_latest_tree_root_pointer_to_stage(root_pointer: TreeHashPointer) {

    let mut file = OpenOptions::new()
        .create(true) // Create the file if it doesn't exist
        .write(true) // Enable writing
        .truncate(true) // Clear file contents before writing
        .open(&stage_file_path())
        .expect("Couldn't open file");

    file.write_all(root_pointer.get_one_hash().as_ref())
        .expect("failed to attach root pointer to stage"); // Writing data
}

pub fn clear_stage() {
    std::fs::write(stage_file_path(), "").expect("Couldn't write to stage file");
}
