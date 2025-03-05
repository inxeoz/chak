
use crate::blob_pointer::{BlobObjectPointer};
use serde::{Deserialize, Serialize};
use crate::blob_object::{BlobObject, CompareOrderStructure};
use crate::version_head_pointer::VersionHeadPointer;
use crate::version_object::VersionObject;
use crate::version_pointer::VersionPointer;

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionHeadObject {
    pointer_to_blob: BlobObjectPointer,
    pointer_to_version: Option<VersionPointer>,
}
impl VersionHeadObject {
    pub fn new(
        pointer_to_blob: BlobObjectPointer,
        pointer_to_version: Option<VersionPointer>,
    ) -> Self {
        Self {
            pointer_to_blob,
            pointer_to_version,
        }
    }
    pub fn get_pointer_to_blob(&self) -> &BlobObjectPointer {
        &self.pointer_to_blob
    }
    pub fn get_pointer_to_version(&self) -> Option<&VersionPointer> {
        self.pointer_to_version.as_ref()
    }

    fn change_blob(&mut self, new_blob_hash: BlobObjectPointer) {
        self.pointer_to_blob = new_blob_hash;
    }
    fn change_version(&mut self, new_version_hash: VersionPointer) {
        self.pointer_to_version = Some(new_version_hash);
    }

    pub fn create_version(&mut self, new_blob_hash: BlobObjectPointer) -> VersionHeadPointer {
        let blob_hashed_content = new_blob_hash.load_blob();
        let previous_blob_hashed_content = self.pointer_to_blob.load_blob();

        let diff_biased_previous = BlobObject::compare_hashed_content_biased_previous(&CompareOrderStructure {
            previous_content: previous_blob_hashed_content,
            new_content: blob_hashed_content,
        });

        let new_version_hashed = VersionObject::new(diff_biased_previous, self.pointer_to_version.clone());
        let latest_version_hash_pointer = VersionPointer::save_version(&new_version_hashed);

        self.change_version(latest_version_hash_pointer);
        self.change_blob(new_blob_hash);

        //creating new version head
        VersionHeadPointer::save_version_head(self)
    }
}