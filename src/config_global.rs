use crate::config::{APPLICATION_NAME};
use crate::util::{deserialize_file_content, serialize_and_save};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::path::{ PathBuf};
pub static MIN_HASH_LENGTH: usize = 3;
pub static CHAK_GLOBAL_CONFIG: &str = "global_config.toml";
pub static CHAK_GLOBAL_CONFIG_ENV: &str = "CHAK_GLOBAL_CONFIG_ENV";
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GlobalConfig {
    pub global_vcs_ignore_file: String,
    pub global_vcs_work_with_nested_ignore_file: bool,
    pub global_vcs_alias: HashMap<String, String>,
}
impl GlobalConfig {
    pub fn new() -> Self {
        GlobalConfig {
            global_vcs_ignore_file: ".ignore".to_string(),
            global_vcs_work_with_nested_ignore_file: false,
            global_vcs_alias: HashMap::new(),
        }
    }

    pub fn create_global_vcs_alias(&mut self, command: String, alias: String) {
        self.global_vcs_alias.insert(command, alias);
    }

    pub fn remove_global_vcs_alias(&mut self, no_need_alias: String) {
        self.global_vcs_alias
            .retain(|current_alias, commnand| current_alias != &no_need_alias);
    }
    pub fn set_global_vcs_work_with_nested_ignore_file(&mut self, value: bool) {
        self.global_vcs_work_with_nested_ignore_file = value;
    }

    pub fn save_global_config(&mut self) -> Result<(), ChakError> {
        let global_config_path = get_global_config_path()?;
        serialize_and_save(&self, &global_config_path)
            .map(|_file| ()) // ignore the File and return ()
            .map_err(|e| e.into())
    }
}

use crate::custom_error::ChakError;
use ::dirs::config_dir;

pub fn get_global_config_path() -> Result<PathBuf, ChakError> {
    if let Ok(env_path) = env::var(CHAK_GLOBAL_CONFIG_ENV) {
        return Ok(PathBuf::from(env_path));
    }

    // Step 2: Fallback to system config directory

    match config_dir() {
        Some(mut config_path) => {
            config_path.push(APPLICATION_NAME);
            config_path.push(CHAK_GLOBAL_CONFIG);
            Ok(config_path)
        }
        None => Err(ChakError::EnvVarNotFound(format!("Environment variable not found {}", CHAK_GLOBAL_CONFIG))),
    }
}
pub fn get_global_config() -> Result<GlobalConfig, ChakError> {
    let config_path = get_global_config_path()?;

    if let Ok(g_config) = deserialize_file_content::<GlobalConfig>(&config_path) {
        Ok(g_config)
    } else {
        // create global config and return
        let mut gc = GlobalConfig::new();
        gc.save_global_config()?;
        Ok(gc)
    }
}
