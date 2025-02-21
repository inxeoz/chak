use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::config::{blob_fold, get_project_dir, trees_fold};
use crate::diff_algo::{hashed_content_from_path, HashedContent};
use crate::handle_common::save_entity;
use crate::handle_tree::TreeObject;
use crate::hashing::{hash_and_content_from_file_path_ref, HashPointer, HashPointerTraits};
use crate::impl_hash_pointer_traits;
use crate::util::{deserialize_file_content, save_or_create_file};

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct BlobHashPointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_traits!(BlobHashPointer);
impl BlobHashPointer {
    pub fn save_blob(hashed_content: HashedContent) -> BlobHashPointer {
        save_entity::<Self, HashedContent>(&hashed_content, &blob_fold())
    }

    pub fn save_blob_from_file(path_to_file: &Path) -> BlobHashPointer {
        let hashed_content = hashed_content_from_path(path_to_file);
        Self::save_blob(hashed_content)
    }



    pub fn load_blob(&self) -> HashedContent {
        let deserialized_content = deserialize_file_content::<HashedContent>(&blob_fold().join(&self.get_path()) ).expect("Failed to load tree file");
        deserialized_content
    }
}
