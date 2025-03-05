use serde::{Deserialize, Serialize};
use crate::blob_object::BlobObject;
use crate::version_pointer::VersionPointer;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct VersionObject {
    pub pointer_to_previous_version: Option<VersionPointer>,
    pub hashed_content: BlobObject,
}
impl VersionObject {
    pub fn new(
        diff_content: BlobObject,
        pointer_to_previous_version: Option<VersionPointer>,
    ) -> Self {
        Self {
            pointer_to_previous_version,
            hashed_content: diff_content,
        }
    }
}