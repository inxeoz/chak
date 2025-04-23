use crate::config_global::GlobalConfig;
use crate::custom_error::ChakError;
use crate::util::{deserialize_file_content, save_or_create_file, serialize_struct};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};

pub static CURRENT_DIR: OnceCell<PathBuf> = OnceCell::new();
pub static CHAK_PATH: OnceCell<PathBuf> = OnceCell::new();

pub static APPLICATION_NAME: &str = "chakvcs";
pub static CHAK_FOLDER_NAME: &str = ".chak/"; //i think if its a folder then it should be suffix with '/'
pub static VCS_CONFIG_NAME: &str = "config";
pub static VCS_IGNORE_FILE_NAME: &str = ".ignore";
pub static REGISTER_NAME: &str = "entries";
pub static mut WORKING_DIR: Option<PathBuf> = None;
pub static BLOBS_DIR_NAME: &str = "blobs";
pub static VERSIONS_DIR_NAME: &str = "versions";
pub static ROOT_TREES_DIR_NAME: &str = "root_trees";
pub static COMMITS_DIR_NAME: &str = "commits";
pub static NESTED_TREES_DIR_NAME: &str = "nested_trees";
pub static VERSION_HEADS_DIR_NAME: &str = "version_heads";
pub static COMMIT_LOG_FILE_NAME: &str = "commit.log";
pub static STAGE_FILE_NAME: &str = "stage";

pub fn get_current_dir_path() -> &'static PathBuf {
    CURRENT_DIR.get_or_init(|| {
        env::current_dir()
            .expect("Could not get current directory")
            .join("aworkspace") // it should be removed while releasing application for deployment or release
    })
}

pub fn get_current_chak_path() -> &'static PathBuf {
    CHAK_PATH.get_or_init(|| {
        get_current_dir_path().join(CHAK_FOLDER_NAME)
    })
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Config {
    pub vcs_work_with_nested_ignore_file: bool,
    pub vcs_command_alias: HashMap<String, String>,
    pub vcs_remotes: HashMap<String, String>,
}

impl Config {
    pub fn new(global_config: &GlobalConfig) -> Self {
        Config {
            vcs_work_with_nested_ignore_file: global_config.global_vcs_work_with_nested_ignore_file,
            vcs_command_alias: global_config.global_vcs_alias.clone(),
            vcs_remotes: HashMap::new(),
        }
    }
    pub fn add_remote(&mut self, remote: String, alias: String) {
        self.vcs_remotes.insert(remote, alias);
    }

    pub fn remove_remote(&mut self, no_need_alias: String) {
        self.vcs_remotes
            .retain(|alias, current_remote| alias != &no_need_alias);
    }
    pub fn create_alias(&mut self, command: String, alias: String) {
        self.vcs_command_alias.insert(command, alias);
    }

    pub fn remove_alias(&mut self, no_need_alias: String) {
        self.vcs_command_alias
            .retain(|current_alias, command| current_alias != &no_need_alias);
    }
    pub fn set_work_with_nested_ignore_file(&mut self, value: bool) {
        self.vcs_work_with_nested_ignore_file = value;
    }
}
pub fn get_chak_fold_path() -> PathBuf {
    get_current_dir_path().join(CHAK_FOLDER_NAME)
}
pub fn get_blob_fold_path() -> PathBuf {
    get_chak_fold_path().join(BLOBS_DIR_NAME)
}
pub fn get_versions_fold_path() -> PathBuf {
    get_chak_fold_path().join(VERSIONS_DIR_NAME)
}
pub fn get_root_trees_fold_path() -> PathBuf {
    get_chak_fold_path().join(ROOT_TREES_DIR_NAME)
}

pub fn get_commits_fold_path() -> PathBuf {
    get_chak_fold_path().join(COMMITS_DIR_NAME)
}

pub fn get_version_head_fold_path() -> PathBuf {
    get_chak_fold_path().join(VERSION_HEADS_DIR_NAME)
}

pub fn get_nested_trees_fold_path() -> PathBuf {
    get_chak_fold_path().join(NESTED_TREES_DIR_NAME)
}



pub fn essentials_folds_to_create() -> Vec<String> {
    vec![
        BLOBS_DIR_NAME.to_string(),
        VERSIONS_DIR_NAME.to_string(),
        ROOT_TREES_DIR_NAME.to_string(),
        COMMITS_DIR_NAME.to_string(),
        NESTED_TREES_DIR_NAME.to_string(),
        COMMITS_DIR_NAME.to_string(),
    ]

}


pub fn get_commit_log_file_path() -> PathBuf {
    get_chak_fold_path().join(COMMIT_LOG_FILE_NAME)
}

pub fn get_stage_file_path() -> PathBuf {
    get_chak_fold_path().join(STAGE_FILE_NAME)
}

pub fn get_config_file_path() -> PathBuf {
    get_chak_fold_path().join(VCS_CONFIG_NAME)
}

pub fn essentials_files_to_create() -> Vec<String> {
    vec![
        STAGE_FILE_NAME.to_string(),
        COMMIT_LOG_FILE_NAME.to_string(),
        VCS_CONFIG_NAME.to_string()
    ]
}

fn _get_file(file_path: &Path) -> Result<File, ChakError> {
    match File::open(file_path) {
        Ok(file) => Ok(file),
        Err(_e) => Err(ChakError::CustomError(format!(
            "Could not open file {}",
            file_path.file_name().unwrap().to_string_lossy()
        ))),
    }
}

//get FILE
pub fn get_commit_log_file() -> Result<File, ChakError> {
    _get_file(&get_commit_log_file_path())
}

pub fn get_stage_file() -> Result<File, ChakError> {
    _get_file(&get_stage_file_path())
}

pub fn get_config_file() -> Result<File, ChakError> {
    _get_file(&get_config_file_path())
}

pub fn get_config() -> Config {
    deserialize_file_content::<Config>(&get_config_file_path())
        .unwrap_or(Config::new(&GlobalConfig::new()))
}

pub fn save_config(config: &Config, project_folder: &Path) -> Result<(), ChakError> {
    let serialized_config = serialize_struct(config)?;
    save_or_create_file(&project_folder.join(VCS_CONFIG_NAME), Some(&serialized_config), false, None)
        .expect("Could not save config");
    Ok(())
}
