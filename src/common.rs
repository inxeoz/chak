use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use crate::tree_object::TreeHashPointer;
use crate::hash_pointer::{HashPointer, HashPointerTraits};
use crate::hash_pointer_algo::{hash_from_content};
use crate::util::{deserialize_file_content, save_or_create_file, serialize_struct};


pub fn save_entity< G: Serialize> (entity: &G, dir_to_save: &Path) ->HashPointer {

    let serialized_content = serialize_struct(&entity);
    let entity_hash = hash_from_content(&serialized_content);

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

pub fn load_entity<T: HashPointerTraits,  S:DeserializeOwned>(entity: &T, parent_dir_of_entity: &Path ) -> S {
    let deserialized_content = deserialize_file_content::<S>(&parent_dir_of_entity.join(entity.get_path()) ).expect("Failed to load file");
    deserialized_content
}

