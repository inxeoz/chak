use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::config::{get_project_dir, Config};
use crate::util::deserialize_file_content;
pub static MIN_HASH_LENGTH: usize = 3;
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GlobalConfig {
    pub global_vcs_ignore_file: String,
    pub global_vcs_work_with_nested_ignore_file: bool,
    pub global_vcs_alias: HashMap<String, String>,

}
impl GlobalConfig {
    pub fn new() -> Self {
        GlobalConfig {
            global_vcs_ignore_file:".ignore".to_string(),
            global_vcs_work_with_nested_ignore_file: false,
            global_vcs_alias: HashMap::new(),

        }
    }

    pub fn create_global_vcs_alias(&mut self, command: String, alias:String) {
        self.global_vcs_alias.insert(command, alias);
    }

    pub fn remove_global_vcs_alias(&mut self, no_need_alias:String) {
        self.global_vcs_alias.retain(|current_alias, commnand| current_alias != &no_need_alias);
    }
    pub fn set_global_vcs_work_with_nested_ignore_file(&mut self, value: bool) {
        self.global_vcs_work_with_nested_ignore_file = value;
    }

}


pub fn get_global_config_path() -> PathBuf {
    PathBuf::from("/.config/chak_vcs/config.toml")
} // for testing purpose create manually this file
pub fn  get_global_config() -> GlobalConfig {
    let g_config = deserialize_file_content::<GlobalConfig>(&get_global_config_path()).expect("Could not read config");
    g_config
}
