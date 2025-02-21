
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use indexmap::IndexMap;
use std::default::Default;
use crate::handle_blob::BlobHashPointer;
use crate::handle_tree::TreeHashPointer;
use crate::handle_version::VersionHashPointer;
use crate::hashing::HashPointerTraits;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum ObjectPointer {
    VersionFile(VersionHashPointer),
    BlobFile(BlobHashPointer),
    TreeFIle(TreeHashPointer),
}

impl ObjectPointer {
    fn get_hash_string(&self) -> String {
        match self {
            ObjectPointer::VersionFile(hash) => {hash.get_one_hash()}
            ObjectPointer::BlobFile(hash) =>  {hash.get_one_hash()}
            ObjectPointer::TreeFIle(hash) => {hash.get_one_hash()}
        }
    }
}
