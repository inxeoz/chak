use crate::blob_pointer::BlobObjectPointer;
use crate::chak_traits::{ChakPointerTraits, HashPointerTraits, ObjectCommonTraits};
use crate::config::get_nested_trees_fold_path;
use crate::custom_error::ChakError;
use crate::nested_tree_pointer::NestedTreeHashPointer;
use crate::version_head_object::VersionHeadObject;
use crate::version_head_pointer::VersionHeadPointer;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NestedTreeObject {
    pub file_children: IndexMap<String, VersionHeadPointer>,
    pub dir_children: IndexMap<String, NestedTreeHashPointer>,
}

impl ObjectCommonTraits for NestedTreeObject {
    fn containing_folder() -> PathBuf {
        get_nested_trees_fold_path()
    }
}
// TreeObject methods
impl NestedTreeObject {
    pub fn from<T: HashPointerTraits>(pointer: &T) -> Result<NestedTreeObject, ChakError> {
        let nested_pointer = NestedTreeHashPointer::own(pointer).map(|v| v.load_tree());
        nested_pointer
    }
    pub fn new() -> NestedTreeObject {
        NestedTreeObject {
            file_children: IndexMap::new(),
            dir_children: IndexMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.file_children.is_empty() && self.dir_children.is_empty()
    }

    pub fn add_dir_child(
        &mut self,
        dir_name: String,
        dir_object: &mut NestedTreeObject,
    ) -> Result<(), ChakError> {
        self.dir_children
            .insert(dir_name, NestedTreeHashPointer::save_tree(dir_object)?);
        Ok(())
    }


    pub fn add_file_child(&mut self, file_entry: &Path, entry_name: &str) -> Result<(), ChakError> {
        let new_blob_hash_pointer = BlobObjectPointer::save_blob_from_path(&file_entry)?;
        if let Some(existing_version) = self.file_children.get_mut(&entry_name.to_string()) {
            let mut version_head = existing_version.load_version_head();
            let updated_version_head_hash_pointer =
                version_head.create_version(new_blob_hash_pointer.clone())?;
            self.file_children
                .insert(entry_name.to_string(), updated_version_head_hash_pointer);

            Ok(())
        } else {
            let new_version_head_hash_pointer = VersionHeadPointer::save_version_head(
                &VersionHeadObject::new(new_blob_hash_pointer, None),
            )?;
            self.file_children
                .insert(entry_name.to_string(), new_version_head_hash_pointer);
            Ok(())
        }
    }

    pub fn sort_children(&mut self) {
        self.file_children.sort_keys();
        self.dir_children.sort_keys();
    }

}
