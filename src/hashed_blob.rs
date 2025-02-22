use crate::hash_pointer::HashPointer;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::config::{blob_fold};
use crate::common::{load_entity, save_entity};
use crate::impl_hash_pointer_traits;
use std::path::PathBuf;
use std::cmp::Ordering;
use std::collections::HashMap;
use indexmap::IndexSet;

use crate::hash_pointer::HashPointerTraits;
pub struct CompareOrderStructure {
    pub previous_content: HashedContent,
    pub new_content: HashedContent,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct HashedContent {
    pub hash_lines: IndexSet<String>,
    pub hash_to_content: HashMap<String, String>,
}


#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct BlobHashPointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_traits!(BlobHashPointer);
impl BlobHashPointer {

    fn own(hash_pointer: &HashPointer) -> BlobHashPointer {
        BlobHashPointer {
            fold_name: hash_pointer.get_fold_name(),
            file_name: hash_pointer.get_file_name(),
        }
    }
    pub fn save_blob(hashed_content: HashedContent) -> BlobHashPointer {
        Self::own(&save_entity::< HashedContent>(&hashed_content, &blob_fold()))
    }

    pub fn save_blob_from_file(path_to_file: &Path) -> BlobHashPointer {
        let hashed_content = HashedContent::hashed_content_from_path(path_to_file);
        Self::save_blob(hashed_content)
    }
    pub fn load_blob(&self) -> HashedContent {
        load_entity::<Self, HashedContent>(self, &blob_fold())
    }

}
