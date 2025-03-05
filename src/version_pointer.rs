use serde::{Deserialize, Serialize};
use crate::common::{load_entity, save_entity};
use crate::config::versions_fold;
use crate::hash_pointer::{HashPointer, HashPointerCommonTraits};
use crate::impl_hash_pointer_common_traits;
use crate::version_object::{ VersionObject};
use std::cmp::Ordering;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct VersionPointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_common_traits!(VersionPointer);

impl VersionPointer {

    fn own(hash_pointer: &HashPointer) -> VersionPointer {
        VersionPointer {
            fold_name: hash_pointer.get_fold_name(),
            file_name: hash_pointer.get_file_name(),
        }
    }


    pub fn save_version(hashed_version: &VersionObject ) -> Self {
        Self::own(&save_entity(hashed_version))
    }

    pub fn load_version(&self) -> VersionObject {
        load_entity::<Self, VersionObject>(self, &versions_fold())
    }
}