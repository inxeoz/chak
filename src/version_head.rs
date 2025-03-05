use crate::config::version_head_fold;
use crate::common::{load_entity, save_entity};
use crate::impl_hash_pointer_common_traits;
use crate::versioning::VersionHead;
use std::path::PathBuf;
use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use crate::hash_pointer::{HashPointer, HashPointerCommonTraits};

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct VersionHeadHashPointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_common_traits!(VersionHeadHashPointer);

impl VersionHeadHashPointer {
    fn own(hash_pointer: &HashPointer) -> VersionHeadHashPointer {
        VersionHeadHashPointer {
            fold_name: hash_pointer.get_fold_name(),
            file_name: hash_pointer.get_file_name(),
        }
    }

    pub fn save_version_head(version_head: &VersionHead) -> VersionHeadHashPointer {
        Self::own(&save_entity(&version_head, &version_head_fold()))
    }
    pub fn load_version_head(&self) -> VersionHead {
        load_entity::<Self, VersionHead>(self, &version_head_fold())
    }
}
