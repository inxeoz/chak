use crate::config::{
    CHAK_FOLDER_NAME, Config, essentials_files_to_create, essentials_folds_to_create,
    get_chak_fold_path, get_current_chak_path, get_current_dir_path, save_config,
};
use crate::util::{input_from_commandline, path_buf_to_name, save_or_create_file};
use std::fs;
use std::fs::create_dir_all;

use crate::config_global::{GlobalConfig, get_global_config};
use crate::custom_error::ChakError;
use colored::Colorize;
use std::path::Path;
use std::string::String;

pub fn handle_command_init(project_name_opt: Option<String>) -> Result<(), ChakError> {
    let chak_folder = {
        if let Some(project_name) = project_name_opt {
            get_current_dir_path()
                .join(&project_name)
                .join(CHAK_FOLDER_NAME)
        } else {
            get_current_chak_path().to_path_buf()
        }
    };

    if chak_folder.exists() {
        println!(
            "cant intialize , there previously .chak folder exist in {}",
            path_buf_to_name(&chak_folder)?.yellow()
        );

        let choice = input_from_commandline(" Do you want to override it? (y/n) ")
            .unwrap_or("n".to_string())
            .to_lowercase();

        if choice == "y" {
            fs::remove_dir_all(&chak_folder)?;
        } else {
            return Err(ChakError::RepoAlreadyExists);
        }
    }

    initialize_vcs(&chak_folder)?;
    println!("{}", "done! .chak/ exist now".green());

    Ok(())
}

pub fn initialize_vcs(project_folder: &Path) -> Result<(), ChakError> {
    for essentials_fold in essentials_folds_to_create() {
        create_dir_all(project_folder.join(essentials_fold))
            .expect("Failed to create Chak folders");
    }
    for essentials_file in essentials_files_to_create() {
        save_or_create_file(&project_folder.join(essentials_file), None, false, None)
            .expect("Failed to create chak files");
    }

    let global_config = get_global_config().unwrap_or(GlobalConfig::new());
    let new_config = Config::new(&global_config);

    save_config(&new_config, project_folder)?;
    Ok(())
}
