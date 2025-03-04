use crate::config::{ nested_trees_fold};

use crate::common::{load_entity, save_entity};
use crate::hash_pointer::{HashPointer, HashPointerOwn, HashPointerTraits};

use crate::custom_error::ChakError;
use crate::impl_hash_pointer_common_traits;
use std::cmp::Ordering;
use std::io;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::nested_tree_object::NestedTreeObject;

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct NestedTreeHashPointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_common_traits!(NestedTreeHashPointer);

impl HashPointerOwn for NestedTreeHashPointer {
    type Output = NestedTreeHashPointer;
    fn own<T: HashPointerTraits>(hash_pointer: &T) -> Result<Self::Output, ChakError> {
        if nested_trees_fold().join(hash_pointer.get_path()).exists() {
            Ok(NestedTreeHashPointer::_own(hash_pointer))
        } else {
            Err(ChakError::CustomError(format!(
                "{}",
                "tree hash pointer not found"
            )))
        }
    }
}
impl NestedTreeHashPointer {
    pub fn save_tree(tree: &mut NestedTreeObject) -> NestedTreeHashPointer {
        tree.sort_children();
        Self::_own(&save_entity::<NestedTreeObject>(tree, &nested_trees_fold()))
    }
    pub fn load_tree(&self) -> NestedTreeObject {
        load_entity::<Self, NestedTreeObject>(self, &nested_trees_fold())
    }

}
