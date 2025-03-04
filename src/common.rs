use crate::hash_pointer::{HashPointer, HashPointerTraits};
use crate::util::{deserialize_file_content, save_or_create_file, serialize_struct};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::create_dir_all;
use std::path::Path;
use crate::config::ENTRY_LIST_FILE;

pub fn load_entity<T: HashPointerTraits, S: DeserializeOwned>(
    entity: &T,
    parent_dir_of_entity: &Path,
) -> S {
    let deserialized_content =
        deserialize_file_content::<S>(&parent_dir_of_entity.join(entity.get_path()))
            .expect("Failed to load file");
    deserialized_content
}
pub fn save_entity<G: Serialize>(entity: &G, dir_to_save: &Path) -> HashPointer {
    let serialized_content = serialize_struct(&entity);
    let entity_hash = HashPointer::from_string(&serialized_content);

    let new_entity_path = dir_to_save.join(entity_hash.get_path()); //error ?

    if !new_entity_path.exists() {
        save_or_create_file(&new_entity_path, Some(&serialized_content), false, None)
            .expect("Could not save blob file");
    }

    //register the new creation of file
    save_or_create_file(
        &dir_to_save.join(ENTRY_LIST_FILE),
        Some(&entity_hash.get_one_hash()),
        true,
        Some("\n"),
    )
    .expect(format!("failed to entry in {}", dir_to_save.display()).as_str());

    entity_hash
}
