use crate::hash_pointer::{HashPointer, HashPointerCommonTraits};
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::config::{blob_fold};
use crate::common::{load_entity, save_entity};
use crate::impl_hash_pointer_common_traits;
use std::path::PathBuf;
use std::cmp::Ordering;
use indexmap::{IndexMap, IndexSet};
use crate::blob_object::BlobObject;
// pub struct CompareOrderStructure {
//     pub previous_content: HashedContent,
//     pub new_content: HashedContent,
// }
// #[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
// pub struct HashedContent {
//     pub hash_lines: IndexSet<String>,
//     pub hash_to_content: IndexMap<String, String>,
// }


#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct BlobObjectPointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_common_traits!(BlobObjectPointer);
impl BlobObjectPointer {

    fn own(hash_pointer: &HashPointer) -> BlobObjectPointer {
        BlobObjectPointer {
            fold_name: hash_pointer.get_fold_name(),
            file_name: hash_pointer.get_file_name(),
        }
    }
    pub fn save_blob(hashed_content: BlobObject) -> BlobObjectPointer {
        Self::own(&save_entity::< BlobObject>(&hashed_content))
    }

    pub fn save_blob_from_file(path_to_file: &Path) -> BlobObjectPointer {
        let hashed_content = BlobObject::hashed_content_from_path(path_to_file);
        Self::save_blob(hashed_content)
    }
    pub fn load_blob(&self) -> BlobObject {
        load_entity::<Self, BlobObject>(self, &blob_fold())
    }

    pub fn exists(&self) -> bool {
        blob_fold().join(self.get_path()).exists()
    }

}
