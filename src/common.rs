use crate::chak_traits::{HashPointerTraits, ObjectCommonTraits};
use crate::config::REGISTER_NAME;
use crate::custom_error::ChakError;
use crate::hash_pointer::HashPointer;
use crate::util::{deserialize_file_content, save_or_create_file};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::path::Path;

pub fn load_entity<T: HashPointerTraits, S: DeserializeOwned>(
    entity: &T,
    parent_dir_of_entity: &Path,
) -> S {
    let deserialized_content =
        deserialize_file_content::<S>(&parent_dir_of_entity.join(entity.get_path()))
            .expect("Failed to load file");
    deserialized_content
}
pub fn save_entity<G: Serialize + ObjectCommonTraits>(entity: &G) -> Result<HashPointer, ChakError> {
    // let serialized_content = serialize_struct(&entity)?;
    // let entity_hash = HashPointer::from_string(&serialized_content);
    let (serialized_content, entity_hash) = entity.serialized_and_hash()?;

    let new_entity_path = G::containing_folder().join(entity_hash.get_path()); //error ?

    if !new_entity_path.exists() {
        save_or_create_file(&new_entity_path, Some(&serialized_content), false, None)?;
    }

    //register the new creation of file
    save_or_create_file(
        &G::containing_folder().join(REGISTER_NAME),
        Some(&entity_hash.get_one_hash()),
        true,
        Some("\n"),
    )?;

    Ok(entity_hash)
}
