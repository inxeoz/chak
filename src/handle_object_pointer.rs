
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use indexmap::IndexMap;
use std::default::Default;
use crate::handle_blob::BlobHashPointer;
use crate::handle_tree::TreeHashPointer;
use crate::handle_version::VersionHashPointer;
use crate::handle_version_head::VersionHeadHashPointer;
use crate::hash_pointer_algo::HashPointerTraits;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ObjectPointer {
    VersionHeadFile(VersionHeadHashPointer),
    // BlobFile(BlobHashPointer),
    TreeFIle(TreeHashPointer),
}

impl ObjectPointer {
    fn get_hash_string(&self) -> String {
        match self {
            ObjectPointer::VersionHeadFile(hash) => {hash.get_one_hash()}
            // ObjectPointer::BlobFile(hash) =>  {hash.get_one_hash()}
            ObjectPointer::TreeFIle(hash) => {hash.get_one_hash()}
        }
    }
}
