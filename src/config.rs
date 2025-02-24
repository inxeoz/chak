use crate::config_global::GlobalConfig;
use crate::custom_error::ChakError;
use crate::util::{deserialize_file_content, save_or_create_file, serialize_struct};
use libc::FILE;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;

pub static CURRENT_DIR: OnceCell<PathBuf> = OnceCell::new();

pub static VCS_FOLDER: &str = ".chak/";
pub static VCS_CONFIG: &str = "config.toml";
pub static VCS_IGNORE_FILE: &str = ".ignore";
pub fn get_project_dir() -> &'static PathBuf {
    CURRENT_DIR.get_or_init(|| {
        env::current_dir()
            .expect("Could not get current directory")
            .join("example") // it should be removed while releasing application for deployment or release
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
pub fn vcs_fold() -> PathBuf {
    get_project_dir().join(VCS_FOLDER)
}
pub fn blob_fold() -> PathBuf {
    vcs_fold().join("blobs")
}
pub fn versions_fold() -> PathBuf {
    vcs_fold().join("versions")
}
pub fn trees_fold() -> PathBuf {
    vcs_fold().join("trees")
}

pub fn commits_fold() -> PathBuf {
    vcs_fold().join("commits")
}

pub fn version_head_fold() -> PathBuf {
    vcs_fold().join("version_heads")
}

pub fn essentials_folds_to_create() -> Vec<PathBuf> {
    vec![
        vcs_fold(),
        blob_fold(),
        versions_fold(),
        trees_fold(),
        commits_fold(),
    ]
}

pub fn commit_log_file_path() -> PathBuf {
    vcs_fold().join("commit.log")
}

pub fn stage_file_path() -> PathBuf {
    vcs_fold().join("stage")
}

pub fn config_file_path() -> PathBuf {
    vcs_fold().join(VCS_CONFIG)
}

pub fn essentials_files_to_create() -> Vec<PathBuf> {
    vec![
        stage_file_path(),
        config_file_path(),
        commit_log_file_path(),
    ]
}

fn _get_file(file_path: &Path) -> Result<File, ChakError> {
    match File::open(file_path) {
        Ok(file) => Ok(file),
        Err(e) => Err(ChakError::CustomError(format!(
            "Could not open file {}",
            file_path.file_name().unwrap().to_string_lossy()
        ))),
    }
}

//get FILE
pub fn get_commit_log_file() -> Result<File, ChakError> {
    _get_file(&commit_log_file_path())
}

pub fn get_stage_file() -> Result<File, ChakError> {
    _get_file(&stage_file_path())
}

pub fn get_config_file() -> Result<File, ChakError> {
    _get_file(&config_file_path())
}

pub fn get_config() -> Config {
    deserialize_file_content::<Config>(&config_file_path())
        .unwrap_or(Config::new(&GlobalConfig::new()))
}

pub fn save_config(config: &Config) {
    let serialized_config = serialize_struct(config);
    save_or_create_file(&config_file_path(), Some(&serialized_config), false, None)
        .expect("Could not save config");
}