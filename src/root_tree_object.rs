
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::path::PathBuf;
use indexmap::IndexMap;
use crate::config::get_root_trees_fold_path;
use crate::custom_error::ChakError;
use crate::root_tree_pointer::{ RootTreePointer};
use crate::nested_tree_pointer::NestedTreeHashPointer;
pub(crate) use crate::nested_tree_object::NestedTreeObject;
use crate::object::ObjectTraits;
use crate::util::{deserialize_file_content};
use crate::version_head_pointer::VersionHeadPointer;
use crate::chak_traits::HashPointerTraits;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RootTreeObject {
    pub file_children: IndexMap<String, VersionHeadPointer>,
    pub dir_children: IndexMap<String, NestedTreeHashPointer>,
}

impl ObjectTraits for RootTreeObject {
    fn containing_folder() -> PathBuf {
        get_root_trees_fold_path()
    }
}
// TreeObject methods
impl RootTreeObject {
    pub fn new() -> RootTreeObject {
        RootTreeObject {
            file_children: IndexMap::new(),
            dir_children: IndexMap::new(),
        }
    }

    pub fn from(nested_tree_object: NestedTreeObject) -> RootTreeObject {
        RootTreeObject {
            file_children: nested_tree_object.file_children.clone(),
            dir_children: nested_tree_object.dir_children.clone(),

        }
    }
    pub fn add_dir_child(&mut self, dir_name: String, nested_dir: &mut NestedTreeHashPointer) {
        self.dir_children.insert(dir_name, nested_dir.clone());
    }
    pub fn add_file_child(&mut self, key: String, value: VersionHeadPointer) {
        self.file_children.insert(key, value);
    }
    pub fn sort_children(&mut self) {
        self.file_children.sort_keys();
        self.dir_children.sort_keys();
    }

    pub fn as_nested_tree(&self) -> NestedTreeObject {
        NestedTreeObject {
            file_children: self.file_children.clone(),
            dir_children: self.dir_children.clone(),
        }
    }

    //root hash pointer writting root tree object

    pub fn get_root_object() -> Result<RootTreeObject, ChakError> {
        // from commit ,getting pointer to previous tree structure that represent the file/folder hierarchy
        match RootTreePointer::get_latest_pointer_from_commit_log() {
            Ok(latest_tree_pointer) => {

                //fetching latest tree from trees fold and converting it to TreeObject so that we can use in our program
                match deserialize_file_content::<RootTreeObject>(&get_root_trees_fold_path().join(latest_tree_pointer.get_path())) {
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

    //TODO how to remove create file and hierarchy before commit
    // pub fn sub(&self, other: &TreeObject) {
    //
    //     let base_file_children = self.file_children.clone();
    //     let base_dir_children = self.dir_children.clone();
    //
    //     let other_file_children = other.file_children.clone();
    //     let other_dir_children = other.dir_children.clone();
    //
    //     let mut isolated_file_children = IndexMap::new();
    //     let mut isolated_dir_children = IndexMap::new();
    //
    //     for base_file_name in base_file_children.keys() {
    //         if ! other_file_children.contains_key(base_file_name) {
    //             if let Some(base_file) = base_file_children.get(base_file_name) {
    //                 isolated_file_children.insert(base_file.to_string(), base_file);
    //             }
    //         }
    //     }
    //
    //     for base_dir_name in base_dir_children.keys() {
    //         if ! other_dir_children.contains_key(base_dir_name) {
    //             if let Some(base_file) = base_dir_children.get(base_dir_name) {
    //                 isolated_dir_children.insert(base_file.to_string(), base_file);
    //             }
    //         }
    //     }
    //
    //
    //
    // }

    // pub fn remove_self_from_hierarchy(&self) {
    //
    //     for (child_name, version_head_pointer) in self.file_children {
    //
    //         if version_head_pointer.load_version_head().get_pointer_to_version().is_none() {
    //
    //         }
    //         let actual_child_file_path = dir_path.join(PathBuf::from(child_name)); //in working folder
    //
    //         let hashed_content = version_head_pointer
    //             .load_version_head()
    //             .get_pointer_to_blob()
    //             .load_blob();
    //         let content = hashed_content.to_string_content();
    //         //save blob data into actual child
    //         save_or_create_file(&actual_child_file_path, Some(&content), false, None)?;
    //     }
    //
    //     for (child_name, existing_tree_object) in self.dir_children {
    //         let actual_child_fold_path = dir_path.join(PathBuf::from(child_name)); //in working folder
    //         if ! actual_child_fold_path.exists() {
    //             create_dir_all(actual_child_fold_path.clone())?;
    //         }
    //         crate::restore::start_restoring(existing_tree_object.clone(), &actual_child_fold_path)?;
    //     }
    //
    //     Ok(())
    //
    // }
}


