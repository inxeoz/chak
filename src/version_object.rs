use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::blob_object::BlobObject;
use crate::chak_traits::ObjectCommonTraits;
use crate::config::{get_versions_fold_path};
use crate::version_pointer::VersionPointer;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct VersionObject {
    pub pointer_to_previous_version: Option<VersionPointer>,
    pub hashed_content: BlobObject,
}

impl ObjectCommonTraits for VersionObject {
    fn containing_folder() -> PathBuf {
        get_versions_fold_path()
    }
}

//version objects methods
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