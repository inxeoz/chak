use crate::config::get_stage_file_path;
use crate::custom_error::ChakError;
use crate::root_tree_pointer::RootTreePointer;
use crate::util::{deserialize_file_content, save_or_create_file, serialize_struct};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Stage {
    pub list_of_root_tree_pointer: Vec<RootTreePointer>,
}

impl Stage {
    pub fn load_stage() -> Result<Stage, ChakError> {
        deserialize_file_content::<Stage>(&get_stage_file_path())
            .map_err(|err| ChakError::CustomError(err.to_string()))
    }

    pub fn add_root_tree_pointer( root_tree_pointer: RootTreePointer) -> Result<(), ChakError> {
        let mut stage = Self::load_stage()?;
        stage.list_of_root_tree_pointer.push(root_tree_pointer);
        let content = serialize_struct(&stage)?;
        save_or_create_file(&get_stage_file_path(), Some(&content), false, None)?;
        Ok(())
    }

    pub fn get_last_root_tree_pointer() -> Option<RootTreePointer> {
        let stage = Self::load_stage().ok()?;
        stage.list_of_root_tree_pointer.last().cloned()
    }


    pub fn get_first_root_tree_pointer() -> Option<RootTreePointer> {
        let stage = Self::load_stage().ok()?;
        stage.list_of_root_tree_pointer.first().cloned()
    }


    pub fn clear_stage() -> Result<(), ChakError> {
        let file_path = get_stage_file_path();

        // Open the file in write mode and truncate it
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(file_path)?;

        // (optional) Write nothing = leave it empty
        //file.write_all(b"")?;
        Ok(())
    }


    pub fn is_stage_clear(  ) -> bool {
        Stage::load_stage()
            .map(|stage| stage.list_of_root_tree_pointer.is_empty())
            .unwrap_or(false)
    }

}
