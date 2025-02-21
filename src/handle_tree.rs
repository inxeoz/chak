use std::fs::{File, OpenOptions};
use std::io::Write;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use crate::config::{commits_fold, get_commit_log_file, get_stage_file, trees_fold, versions_fold};
use crate::diff::HashedContentForVersion;
use crate::diff_algo::HashedContentForVersion;
use crate::handle_commit::{Commit, CommitHashPointer};
use crate::handle_common::save_entity;
use crate::hashing::{get_latest_pointer_line_from_file, hash_from_content, HashPointer, HashPointerTraits};
use crate::impl_hash_pointer_traits;
use crate::handle_object_pointer::ObjectPointer;
use crate::util::{deserialize_file_content, save_or_create_file, serialize_struct};

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct TreeHashPointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_traits!(TreeHashPointer);
impl TreeHashPointer {
    pub fn save_tree( tree: &mut TreeObject) -> TreeHashPointer {
        tree.sort_children();
        save_entity::<Self, TreeObject>(tree, &trees_fold())

    }
    pub fn load_tree(self) -> TreeObject {
        let deserialized_content = deserialize_file_content::<TreeObject>(&trees_fold().join(self.get_path()) ).expect("Failed to load tree file");
        deserialized_content
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

pub fn get_latest_tree_root_pointer(from_stage: bool) -> TreeHashPointer{

    if from_stage && get_stage_file().exists(){
        let stage_file = File::open(get_stage_file()).expect("failed to open stage file");
        let latest_tree_pointer = get_latest_pointer_line_from_file(&stage_file, true).expect("failed to get latest pointer");
        latest_tree_pointer
    }else {
        let commit_log_file = File::open(get_commit_log_file()).expect("failed to open commit log  file");
        let latest_commit_pointer = get_latest_pointer_line_from_file::<CommitHashPointer>(&commit_log_file, true).expect("failed to get latest pointer");
        let latest_commit = deserialize_file_content::<Commit>(&commits_fold().join(latest_commit_pointer.get_path())).expect("failed to deserialize commit");
        latest_commit.root_tree_pointer
    }
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
}
