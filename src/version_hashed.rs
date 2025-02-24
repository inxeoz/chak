use serde::{Deserialize, Serialize};
use crate::config::versions_fold;
use crate::common::{load_entity, save_entity};
use crate::impl_hash_pointer_common_traits;
use std::path::PathBuf;
use std::cmp::Ordering;
use crate::blob_hash_pointer::{BlobHashPointer, HashedContent};
use crate::hash_pointer::{HashPointer, HashPointerTraits};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct VersionHashedContent {
    pub pointer_to_previous_version: Option<VersionHashPointer>,
    pub hashed_content: HashedContent,
}
impl VersionHashedContent {
    pub fn new(
        diff_content: HashedContent,
        pointer_to_previous_version: Option<VersionHashPointer>,
    ) -> Self {
        Self {
            pointer_to_previous_version,
            hashed_content: diff_content,
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct VersionHashPointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_common_traits!(VersionHashPointer);

impl VersionHashPointer {

    fn own(hash_pointer: &HashPointer) -> VersionHashPointer {
        VersionHashPointer {
            fold_name: hash_pointer.get_fold_name(),
            file_name: hash_pointer.get_file_name(),
        }
    }


    pub fn save_version(hashed_version: &VersionHashedContent ) -> Self {
        Self::own(&save_entity(hashed_version, &versions_fold()))
    }

    pub fn load_version(&self) -> VersionHashedContent {
        load_entity::<Self, VersionHashedContent>(self, &versions_fold())
    }
}