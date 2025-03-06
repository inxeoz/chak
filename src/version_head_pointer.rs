use crate::restricted;
use crate::chak_traits::HashPointerTraits;
use crate::config::version_head_fold;
use crate::common::{load_entity, save_entity};
use crate::impl_hash_pointer_common_traits;
use crate::version_head_object::{ VersionHeadObject};
use std::path::PathBuf;
use std::cmp::Ordering;
use serde::{Deserialize, Serialize};
use crate::chak_traits::ChakPointerTraits;
use crate::custom_error::ChakError;

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct VersionHeadPointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_common_traits!(VersionHeadPointer, VersionHeadObject);
impl VersionHeadPointer {

    pub fn save_version_head(version_head: &VersionHeadObject) -> Result<VersionHeadPointer, ChakError> {
        Self::own(&save_entity(version_head))
    }
    pub fn load_version_head(&self) -> VersionHeadObject {
        load_entity::<Self, VersionHeadObject>(self, &version_head_fold())
    }
}
