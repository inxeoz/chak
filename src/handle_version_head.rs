use crate::config::version_head_fold;
use crate::handle_common::{load_entity, save_entity};
use crate::impl_hash_pointer_traits;
use crate::versioning::VersionHead;

pub struct VersionHeadHashPointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_traits!(VersionHeadHashPointer);

impl VersionHeadHashPointer {

    pub fn save_version_head(version_head: &VersionHead) -> VersionHeadHashPointer {
        save_entity(&version_head, &version_head_fold())
    }
    pub fn load_version_head(&self) -> VersionHead {
        load_entity::<Self, VersionHead>(self, &version_head_fold())
    }
}
