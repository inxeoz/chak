
use serde::{Deserialize, Serialize};
use std::default::Default;
use indexmap::IndexMap;
use crate::config::trees_fold;
use crate::custom_error::ChakError;
use crate::tree_hash_pointer::TreeHashPointer;
use crate::version_head::VersionHeadHashPointer;
use crate::hash_pointer::HashPointerTraits;
use crate::util::deserialize_file_content;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TreeObject {
    pub file_children: IndexMap<String, VersionHeadHashPointer>,
    pub dir_children: IndexMap<String, TreeObject>,
}
// TreeObject methods
impl TreeObject {
    pub fn new() -> TreeObject {
        TreeObject {
            file_children: IndexMap::new(),
            dir_children: IndexMap::new(),
            // file_children: Default::default(),
            // dir_children: Default::default(),
        }
    }
    pub fn add_dir_child(&mut self, dir_name: String, dir_object: TreeObject) {
        self.dir_children.insert(dir_name, dir_object);
    }
    pub fn add_file_child(&mut self, key: String, value: VersionHeadHashPointer) {
        self.file_children.insert(key, value);
    }
    pub fn sort_children(&mut self) {
        self.file_children.sort_keys();
        self.dir_children.sort_keys();
    }

    pub fn get_top_most_tree_object() -> Result<TreeObject, ChakError> {
        // from commit ,getting pointer to previous tree structure that represent the file/folder hierarchy
        match TreeHashPointer::get_latest_pointer_from_commit_log() {
            Ok(latest_tree_pointer) => {

                //fetching latest tree from trees fold and converting it to TreeObject so that we can use in our program
                match deserialize_file_content::<TreeObject>(&trees_fold().join(latest_tree_pointer.get_path())) {
                    Ok(tree_object_s) => {
                        Ok(tree_object_s)
                    }
                    Err(e) => {
                        Err(ChakError::from(e))
                    }
                }
            }
            Err(e) => {
                Err(e.into())
            }
        }
    }
}

