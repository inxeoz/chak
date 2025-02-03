use crate::hashing::HashPointer;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ffi::OsString;
use indexmap::IndexMap;
use itertools::Itertools;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum TreeObjectType {
    Diff,
    BlobFile,
    TreeObject,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TreeObject {
    pub children: IndexMap<String, TreeNode>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct TreeNode {
    pub blob_type: TreeObjectType,
    pub pointer_to_blob: HashPointer,
    pub pointer_to_diff: Option<HashPointer>,
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.pointer_to_blob.get_one_hash() == other.pointer_to_blob.get_one_hash()
    }
}

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pointer_to_blob.get_one_hash().cmp(&other.pointer_to_blob.get_one_hash())
    }
}

impl PartialOrd for TreeNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TreeObject {
    pub fn sort_children(&mut self) {
        self.children.sort_keys()
    }
}
