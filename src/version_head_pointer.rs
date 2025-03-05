use crate::config::version_head_fold;
use crate::common::{load_entity, save_entity};
use crate::impl_hash_pointer_common_traits;
use crate::version_head_object::{ VersionHeadObject};
use std::path::PathBuf;
use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use crate::hash_pointer::{HashPointer, HashPointerCommonTraits};

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct VersionHeadPointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_common_traits!(VersionHeadPointer);

impl VersionHeadPointer {
    fn own(hash_pointer: &HashPointer) -> VersionHeadPointer {
        VersionHeadPointer {
            fold_name: hash_pointer.get_fold_name(),
            file_name: hash_pointer.get_file_name(),
        }
    }

    pub fn save_version_head(version_head: &VersionHeadObject) -> VersionHeadPointer {
        Self::own(&save_entity(version_head))
    }
    pub fn load_version_head(&self) -> VersionHeadObject {
        load_entity::<Self, VersionHeadObject>(self, &version_head_fold())
    }
}
