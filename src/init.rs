use crate::config::{essentials_files_to_create, essentials_folds_to_create, get_current_dir_path, save_config, get_chak_fold_path, Config, CHAK_FOLDER_NAME};
use crate::util::{input_from_commandline, save_or_create_file};
use std::fs::create_dir_all;
use std::{fs};

use std::path::Path;
use crate::config_global::{get_global_config, GlobalConfig};
use std::string::String;
use crate::custom_error::ChakError;

pub fn init(project_name: Option<String>) -> Result<(), ChakError> {

    let project_folder = {
        if let Some( project_name) = project_name {

            //cause project would dir first for that we will append slace suffix in project name
            let project_folder = &get_current_dir_path().join( &project_name);

            if project_folder.exists() {
                println!("can't intialize there exist {} dir prviously exist ", &project_name);
                return Err(ChakError::RepoAlreadyExists)
            }else {
                create_dir_all( project_folder)?;
                println!("start hacking\ncd {}\nchak status", &project_name);
            }

           project_folder.to_owned()

        }else {

            if get_chak_fold_path().exists() {
                // checking .chak/ folder exist or not

                let choice = input_from_commandline(
                    "A '.chak' folder already exists in this directory. Do you want to override it? (y/n) ",
                )
                    .unwrap_or("n".to_string());

                if choice.to_lowercase() != "y" {
                    println!("Operation canceled. The '.chak' folder remains unchanged.");
                    return Ok(());
                } else {
                    fs::remove_dir_all(get_chak_fold_path())?; // removing previously exist .chak/ folder
                    println!("Initializing The '.chak' folder");
                }
            } else {
                println!(
                    "Initializing empty Chak repository in '{}'",
                    get_chak_fold_path().display()
                );
            }

            get_current_dir_path().to_owned()
        }
    };

    //this is an entry point for intialization of version control system chak
    initialize_vcs(&project_folder.join(CHAK_FOLDER_NAME))?;
    println!("done! .chak/ exist now");

    Ok(())
}

pub fn initialize_vcs(project_folder: &Path) -> Result<(), ChakError> {
    println!("--> project folder {}", project_folder.display());

    for essentials_fold in essentials_folds_to_create() {

        create_dir_all(project_folder.join(essentials_fold)).expect("Failed to create Chak folders");
    }
    for essentials_file in essentials_files_to_create() {
        save_or_create_file(&project_folder.join(essentials_file), None, false, None)
            .expect("Failed to create chak files");
    }

    let global_config = get_global_config().unwrap_or(GlobalConfig::new());
    let new_config = Config::new(&global_config);

    save_config(&new_config, project_folder)?;
    Ok(
        ()
    )

}
