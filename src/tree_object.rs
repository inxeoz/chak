
use serde::{Deserialize, Serialize};
use std::default::Default;
use indexmap::IndexMap;
use crate::config::trees_fold;
use crate::custom_error::ChakError;
use crate::tree_object::TreeHashPointer;
use crate::version_head::VersionHeadHashPointer;
use crate::hash_pointer::HashPointerTraits;
use crate::util::deserialize_file_content;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ObjectPointer {
    VersionHeadFile(VersionHeadHashPointer),
    // BlobFile(BlobHashPointer),
    TreeFIle(TreeHashPointer),
}

impl ObjectPointer {
    fn get_hash_string(&self) -> String {
        match self {
            ObjectPointer::VersionHeadFile(hash) => {hash.get_one_hash()}
            // ObjectPointer::BlobFile(hash) =>  {hash.get_one_hash()}
            ObjectPointer::TreeFIle(hash) => {hash.get_one_hash()}
        }
    }
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

    pub fn get_top_most_tree_object() -> Result<TreeObject, ChakError> {
        // from commit ,getting pointer to previous tree structure that represent the file/folder hierarchy

        match TreeHashPointer::get_latest_tree_root_pointer(true)  {
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

