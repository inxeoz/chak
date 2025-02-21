use std::path::{Path, PathBuf};
use serde::Serialize;
use crate::config::trees_fold;
use crate::handle_tree::{TreeHashPointer, TreeObject};
use crate::hashing::{hash_from_content, HashPointer, HashPointerTraits};
use crate::util::{save_or_create_file, serialize_struct};

pub fn save_entity<T: HashPointerTraits, G: Serialize> (entity: &G, dir_to_save: &Path) -> T {

    let serialized_content = serialize_struct(&entity);
    let entity_hash = hash_from_content::<T>(&serialized_content);

    let new_entity_path  = dir_to_save.join(entity_hash.get_path()); //error

    if !new_entity_path.exists() {
        save_or_create_file(
            &new_entity_path,
            Some(&serialized_content),
            false,
            None,
        )
            .expect("Could not save blob file");
    }

   entity_hash

}