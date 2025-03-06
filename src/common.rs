use crate::hash_pointer::{HashPointer};
use crate::util::{deserialize_file_content, save_or_create_file, serialize_struct};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::Path;
use crate::chak_traits::HashPointerTraits;
use crate::config::REGISTER;
use crate::object::ObjectTraits;

pub fn load_entity<T: HashPointerTraits, S: DeserializeOwned>(
    entity: &T,
    parent_dir_of_entity: &Path,
) -> S {
    let deserialized_content =
        deserialize_file_content::<S>(&parent_dir_of_entity.join(entity.get_path()))
            .expect("Failed to load file");
    deserialized_content
}
pub fn save_entity<G: Serialize + ObjectTraits>(entity: &G) -> HashPointer {
    let serialized_content = serialize_struct(&entity);
    let entity_hash = HashPointer::from_string(&serialized_content);

   // let m_path = entity

    let new_entity_path = G::containing_folder().join(entity_hash.get_path()); //error ?

    if !new_entity_path.exists() {
        save_or_create_file(&new_entity_path, Some(&serialized_content), false, None)
            .expect("Could not save blob file");
    }

    //register the new creation of file
    save_or_create_file(
        &G::containing_folder().join(REGISTER),
        Some(&entity_hash.get_one_hash()),
        true,
        Some("\n"),
    )
    .expect(format!("failed to entry in {}", G::containing_folder().display()).as_str());

    entity_hash
}
