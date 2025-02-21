use serde::{Deserialize, Serialize};
use crate::config::versions_fold;
use crate::diff_algo::HashedContentForVersion;
use crate::handle_common::save_entity;
use crate::impl_hash_pointer_traits;
use crate::util::{deserialize_file_content, serialize_struct};

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct VersionHashPointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_traits!(VersionHashPointer);

impl VersionHashPointer {
    pub fn save_version(hashed_version: &HashedContentForVersion ) -> Self {
        save_entity::<Self, HashedContentForVersion>(hashed_version, &versions_fold())
    }

    pub fn load_version(&self) -> HashedContentForVersion {
        let deserialized_content = deserialize_file_content::<HashedContentForVersion>(&versions_fold().join(self.get_path()) ).expect("Failed to load version file");
        deserialized_content
    }
}