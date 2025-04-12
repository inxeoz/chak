use crate::restricted;
use crate::chak_traits::HashPointerTraits;
use serde::{Deserialize, Serialize};
use crate::common::{load_entity, save_entity};
use crate::config::get_versions_fold_path;
use crate::impl_hash_pointer_common_traits;
use crate::version_object::{ VersionObject};
use std::cmp::Ordering;
use crate::chak_traits::ChakPointerTraits;
use crate::custom_error::ChakError;

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct VersionPointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_common_traits!(VersionPointer, VersionObject);

impl VersionPointer {

    pub fn save_version(hashed_version: &VersionObject ) -> Result<VersionPointer, ChakError> {
        Self::own(&save_entity(hashed_version)?)
    }

    pub fn load_version(&self) -> VersionObject {
        load_entity::<Self, VersionObject>(self, &get_versions_fold_path())
    }
}