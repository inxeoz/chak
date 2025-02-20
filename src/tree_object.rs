use crate::hashing::{BlobHashPointer, HashPointer, HashPointerTraits, TreeHashPointer, VersionHashPointer};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use indexmap::IndexMap;

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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TreeNode {
    pub hash_pointer_to_this_node: ObjectPointer,
    pub hash_pointer_to_previous_version: Option<VersionHashPointer>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TreeObject {
    pub children: IndexMap<String, TreeNode>,
}

// PartialEq: Compare based on hash_pointer_to_this_node
impl PartialEq for TreeNode
{
    fn eq(&self, other: &Self) -> bool {
        self.hash_pointer_to_this_node == other.hash_pointer_to_this_node
    }
}

// Eq: Marker trait, relies on PartialEq
impl Eq for TreeNode {}

// PartialOrd: Delegates to Ord
impl PartialOrd for TreeNode
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Ord: Compare based on hash_pointer_to_this_node
impl Ord for TreeNode
{
    fn cmp(&self, other: &Self) -> Ordering {


        self.hash_pointer_to_this_node.get_hash_string().cmp(&other.hash_pointer_to_this_node.get_hash_string())
    }
}

// TreeObject methods
impl TreeObject {
    pub fn sort_children(&mut self) {
        self.children.sort_keys();
    }
}