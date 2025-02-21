use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::config::{blob_fold, get_project_dir, trees_fold};
use crate::diff_algo::{ HashedContent};
use crate::handle_common::{load_entity, save_entity};
use crate::handle_tree::TreeObject;
use crate::hash_pointer_algo::{hash_and_content_from_file_path_ref, HashPointer, HashPointerTraits};
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
        let hashed_content = HashedContent::hashed_content_from_path(path_to_file);
        Self::save_blob(hashed_content)
    }
    pub fn load_blob(&self) -> HashedContent {
        load_entity::<Self, HashedContent>(self, &blob_fold())
    }
}
