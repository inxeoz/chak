use crate::config::{commits_fold, get_commit_log_file, get_stage_file, trees_fold, versions_fold};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::Write;

use crate::commit::{Commit, CommitHashPointer};
use crate::common::{load_entity, save_entity};
use crate::object_pointer::ObjectPointer;
use crate::hash_pointer::{HashPointer, HashPointerTraits};
use crate::hash_pointer_algo::{
    get_latest_pointer_line_from_file, hash_from_content,
};
use std::path::PathBuf;
use std::cmp::Ordering;


use crate::impl_hash_pointer_traits;
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

    pub fn get_latest_tree_root_pointer(from_stage: bool) -> Option<TreeHashPointer> {
        if from_stage && get_stage_file().exists() {
            let stage_file = File::open(get_stage_file()).expect("failed to open stage file");
            if let Some(latest_tree_pointer) = get_latest_pointer_line_from_file(&stage_file, true)
            {
                Some(Self::own(&latest_tree_pointer))
            } else {
                None
            }
        } else {
            let commit_log_file =
                File::open(get_commit_log_file()).expect("failed to open commit log  file");
            if let Some(latest_commit_pointer) =
                get_latest_pointer_line_from_file(&commit_log_file, true)
            {
                let latest_commit = deserialize_file_content::<Commit>(
                    &commits_fold().join(latest_commit_pointer.get_path()),
                )
                .expect("failed to deserialize commit");
                Some(latest_commit.root_tree_pointer)
            } else {
                None
            }
        }
    }
}
pub fn attach_latest_tree_root_pointer_to_stage(root_pointer: TreeHashPointer) {
    let mut file = OpenOptions::new()
        .create(true) // Create the file if it doesn't exist
        .write(true) // Enable writing
        .truncate(true) // Clear file contents before writing
        .open(&get_stage_file())
        .expect("Couldn't open file");

    file.write_all(root_pointer.get_one_hash().as_ref())
        .expect("failed to attach root pointer to stage"); // Writing data
}

pub fn clear_stage() {
    std::fs::write(get_stage_file(), "").expect("Couldn't write to stage file");
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TreeObject {
    pub children: IndexMap<String, ObjectPointer>,
}
// TreeObject methods
impl TreeObject {
    pub fn new() -> TreeObject {
        TreeObject {
            children: IndexMap::new(),
        }
    }
    pub fn add_child(&mut self, key: String, value: ObjectPointer) {
        self.children.insert(key, value);
    }
    pub fn sort_children(&mut self) {
        self.children.sort_keys();
    }

    pub fn get_top_most_tree_object() -> Option<TreeObject> {
        // from commit ,getting pointer to previous tree structure that represent the file/folder hierarchy
        if let Some(root_tree_pointer) = TreeHashPointer::get_latest_tree_root_pointer(false) {
            //fetching latest tree from trees fold and converting it to TreeObject so that we can use in our program
            match deserialize_file_content::<TreeObject>(&trees_fold().join(root_tree_pointer.get_path())) {
                Ok(tree_object_s) => {
                    Some(tree_object_s)
                }
                Err(e) => {
                    eprintln!("failed to deserialize root tree pointer  {}", e);
                    None
                }
            }
        }else {
            None
        }
    }
}
