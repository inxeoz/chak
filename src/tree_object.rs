
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

impl TreeObject {
    pub fn sort_children( self: &mut Self) {
        self.children.sort();
    }
}