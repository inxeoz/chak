use crate::hashing::HashPointer;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use indexmap::IndexMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum TreeObjectType {
    Diff,
    BlobFile,
    TreeObject,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TreeObject {
    pub children: IndexMap<String, TreeNode>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct TreeNode {
    pub node_type: TreeObjectType,
    pub hash_pointer_to_this_node: HashPointer,
    pub hash_pointer_to_previous_version: Option<HashPointer>,
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.hash_pointer_to_this_node == other.hash_pointer_to_this_node
    }
}

impl Ord for TreeNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hash_pointer_to_this_node.cmp(&other.hash_pointer_to_this_node)
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
