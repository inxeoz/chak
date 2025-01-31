
use crate::hashing::HashPointer;
use std::hash::Hash;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum TreeObjectType {
    Diff,
    BlobFile,
    TreeObject,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeObject {
    pub tree_object_type: TreeObjectType,
    pub children:Vec<HashPointer>,
}