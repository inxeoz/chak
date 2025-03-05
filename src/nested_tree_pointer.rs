use crate::config::{commits_fold, nested_trees_fold};

use crate::common::{load_entity, save_entity};


use crate::custom_error::ChakError;
use crate::impl_hash_pointer_common_traits;
use std::cmp::Ordering;
use std::io;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::commit_pointer::CommitHashPointer;
use crate::hash_pointer::{HashPointerCommonTraits, HashPointerCoreTraits};
use crate::nested_tree_object::NestedTreeObject;

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct NestedTreeHashPointer {
    fold_name: String,
    file_name: String,
}
impl_hash_pointer_common_traits!(NestedTreeHashPointer);

impl HashPointerCoreTraits for NestedTreeHashPointer {
    type Output = NestedTreeHashPointer;


    fn own<T: HashPointerCommonTraits>(hash_pointer: &T) -> Result<Self::Output, ChakError> {
        if Self::verify_existing(hash_pointer) {
            Ok(NestedTreeHashPointer {
                file_name: hash_pointer.get_file_name(),
                fold_name: hash_pointer.get_fold_name(),
            })
        } else {
            Err(ChakError::CustomError(format!(
                "{}",
                "tree hash pointer not found"
            )))
        }
    }

    fn verify_existing<T: HashPointerCommonTraits>(hash_pointer: &T) -> bool {
        nested_trees_fold().join(hash_pointer.get_path()).exists()
    }


}
impl NestedTreeHashPointer {
    pub fn save_tree(tree: &mut NestedTreeObject) -> Result<NestedTreeHashPointer, ChakError> {
        tree.sort_children();
        Self::own(&save_entity(tree))
    }
    pub fn load_tree(&self) -> NestedTreeObject {
        load_entity::<Self, NestedTreeObject>(self, &nested_trees_fold())
    }

}
