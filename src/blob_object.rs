use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use indexmap::{IndexMap, IndexSet};
use crate::config::get_blob_fold_path;
use crate::object::ObjectTraits;

pub struct CompareOrderStructure {
    pub previous_content: BlobObject,
    pub new_content: BlobObject,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BlobObject {
    pub hash_lines: IndexSet<String>,
    pub hash_to_content: IndexMap<String, String>,
}

impl ObjectTraits for BlobObject {
    fn containing_folder() -> PathBuf {
        get_blob_fold_path()
    }
}

