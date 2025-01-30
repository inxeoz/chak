use crate::diff_algo::HashedContent;
use crate::hashing::HashPointer;
use std::hash::Hash;
use std::ptr::hash;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ObjectType {
    Diff,
    BlobFile,
    TreeObject,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Object {
    pub object_type: ObjectType,
    pub hash_pointer: HashPointer,
    pub children: Option<Vec<Object>>,
}

impl Object {
    pub fn new(object_type: ObjectType, hash_pointer: HashPointer) -> Object {
        Object {
            object_type,
            hash_pointer,
            children: None,
        }
    }

    pub fn from( children: Vec<Object>) -> Object {

        Object {
            object_type: ObjectType::TreeObject,
            hash_pointer,
            children: Some(children),
        }
    }

    pub fn add_child(&mut self, child: Object) {
        self.hash_pointer
            .update_hash(child.hash_pointer.get_one_hash());
        if let Some(ref mut children) = self.children {
            children.push(child);
        } else {
            self.children = Some(vec![child]);
            self.object_type = ObjectType::TreeObject;
        }
    }
}
