use serde::{Deserialize, Serialize};
use crate::config::versions_fold;
use crate::diff_algo::HashedContentForVersion;
use crate::handle_common::{load_entity, save_entity};
use crate::impl_hash_pointer_traits;
use std::path::PathBuf;
use std::cmp::Ordering;
use crate::handle_blob::BlobHashPointer;
use crate::hash_pointer_algo::hash_from_content;
use crate::hash_pointer::{HashPointer, HashPointerTraits};
#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct VersionHashPointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_traits!(VersionHashPointer);

impl VersionHashPointer {

    fn own(hash_pointer: &HashPointer) -> VersionHashPointer {
        VersionHashPointer {
            fold_name: hash_pointer.get_fold_name(),
            file_name: hash_pointer.get_file_name(),
        }
    }


    pub fn save_version(hashed_version: &HashedContentForVersion ) -> Self {
        Self::own(&save_entity(hashed_version, &versions_fold()))
    }

    pub fn load_version(&self) -> HashedContentForVersion {
        load_entity::<Self, HashedContentForVersion>(self, &versions_fold())
    }
}