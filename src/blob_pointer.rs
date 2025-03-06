use crate::restricted;
use crate::chak_traits::{ HashPointerTraits};
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::config::{blob_fold};
use crate::common::{load_entity, save_entity};
use crate::impl_hash_pointer_common_traits;
use std::path::PathBuf;
use std::cmp::Ordering;
use indexmap::{IndexMap, IndexSet};
use crate::blob_object::BlobObject;
use crate::chak_traits::ChakPointerTraits;
use crate::commit_object::CommitObject;
use crate::commit_pointer::CommitPointer;
use crate::custom_error::ChakError;
// pub struct CompareOrderStructure {
//     pub previous_content: HashedContent,
//     pub new_content: HashedContent,
// }
// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// pub struct HashedContent {
//     pub hash_lines: IndexSet<String>,
//     pub hash_to_content: IndexMap<String, String>,
// }


#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct BlobObjectPointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_common_traits!(BlobObjectPointer, BlobObject);

impl BlobObjectPointer {

    pub fn save_blob(hashed_content: BlobObject) -> Result<BlobObjectPointer, ChakError> {
        Self::own(&save_entity::< BlobObject>(&hashed_content))
    }

    pub fn save_blob_from_file(path_to_file: &Path) -> Result<BlobObjectPointer, ChakError> {
        let hashed_content = BlobObject::hashed_content_from_path(path_to_file);
        Self::save_blob(hashed_content)
    }
    pub fn load_blob(&self) -> BlobObject {
        load_entity::<Self, BlobObject>(self, &blob_fold())
    }

    pub fn exists(&self) -> bool {
        blob_fold().join(self.get_path()).exists()
    }

}
