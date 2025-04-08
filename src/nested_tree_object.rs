
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::path::{Path, PathBuf};
use indexmap::IndexMap;
use crate::blob_pointer::BlobObjectPointer;
use crate::config::nested_trees_fold;
use crate::custom_error::ChakError;
use crate::nested_tree_pointer::NestedTreeHashPointer;
use crate::object::ObjectTraits;
use crate::version_head_object::VersionHeadObject;
use crate::version_head_pointer::VersionHeadPointer;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NestedTreeObject {
    pub file_children: IndexMap<String, VersionHeadPointer>,
    pub dir_children: IndexMap<String, NestedTreeHashPointer>,
}

impl ObjectTraits for NestedTreeObject {
    fn containing_folder() -> PathBuf {
        nested_trees_fold()
    }
}
// TreeObject methods
impl NestedTreeObject {
    pub fn new() -> NestedTreeObject {
        NestedTreeObject {
            file_children: IndexMap::new(),
            dir_children: IndexMap::new(),
            // file_children: Default::default(),
            // dir_children: Default::default(),
        }
    }

    //why we have not configured add_dir_child automatically ???
    //because this adding directory depeneds on how to define and works the directory
    //
    pub fn add_dir_child(&mut self, dir_name: String, dir_object: &mut NestedTreeObject) -> Result<(), ChakError> {
        self.dir_children.insert(dir_name, NestedTreeHashPointer::save_tree(dir_object)?);
        Ok(())
    }


    // pub fn save_version_head(version_head: &VersionHead) -> VersionHeadHashPointer {
    //     Self::own(&save_entity(&version_head, &version_head_fold()))
    // }


    // let updated_version_head_hash_pointer =
    // version_head.create_version(blob_hash_pointer.clone());

    // if let Some(existing_version) = tree_ref.file_children.get(&entry_name.to_string()) {
    // let mut version_head = existing_version.load_version_head();
    // let updated_version_head_hash_pointer =
    // version_head.create_version(blob_hash_pointer.clone());

    pub fn add_file_child(&mut self, file_entry: &Path, entry_name: &str) -> Result<(), ChakError> {

        let new_blob_hash_pointer = BlobObjectPointer::save_blob_from_path(&file_entry)?;
        if let Some( existing_version) = self.file_children.get_mut(&entry_name.to_string()) {

            let mut version_head = existing_version.load_version_head();
            let updated_version_head_hash_pointer =
                version_head.create_version(new_blob_hash_pointer.clone())?;
            self.file_children.insert(entry_name.to_string(), updated_version_head_hash_pointer);

            Ok(())

           // if was_it_registered(existing_version.clone(), &version_head_fold()) {
           //
           // }

        } else {
            let new_version_head_hash_pointer =
                VersionHeadPointer::save_version_head(&VersionHeadObject::new(new_blob_hash_pointer, None))?;
            self.file_children.insert(entry_name.to_string(), new_version_head_hash_pointer);
            Ok(())
        }
    }

    pub fn sort_children(&mut self) {
        self.file_children.sort_keys();
        self.dir_children.sort_keys();
    }

    // pub fn get_top_most_tree_object() -> Result<NestedTreeObject, ChakError> {
    //     // from commit ,getting pointer to previous tree structure that represent the file/folder hierarchy
    //     match RootTreeHashPointer::get_latest_pointer_from_commit_log() {
    //         Ok(latest_tree_pointer) => {
    //
    //             //fetching latest tree from trees fold and converting it to TreeObject so that we can use in our program
    //             match deserialize_file_content::<NestedTreeObject>(&root_trees_fold().join(latest_tree_pointer.get_path())) {
    //                 Ok(tree_object_s) => {
    //                     Ok(tree_object_s)
    //                 }
    //                 Err(e) => {
    //                     Err(ChakError::from(e))
    //                 }
    //             }
    //         }
    //         Err(e) => {
    //             Err(e.into())
    //         }
    //     }
    // }

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


