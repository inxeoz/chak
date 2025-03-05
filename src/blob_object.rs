
use serde::{Deserialize, Serialize};

use indexmap::{IndexMap, IndexSet};
pub struct CompareOrderStructure {
    pub previous_content: BlobObject,
    pub new_content: BlobObject,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BlobObject {
    pub hash_lines: IndexSet<String>,
    pub hash_to_content: IndexMap<String, String>,
}

