use crate::config::{
    blob_fold, commits_fold, get_project_dir, get_vcs_fold, history_fold, staging_area_fold,
};
use crate::macros::{ input_from_commandline, save_or_create_file};
use std::fs::create_dir_all;
use std::{fs, io};

pub fn init() -> Result<(), io::Error> {
    if get_vcs_fold().exists() {
        let choice = input_from_commandline(
            "A '.chak' folder already exists in this directory. Do you want to override it? (y/n) ",
        ).unwrap_or("n".to_string());

        if choice == "y" {
            fs::remove_dir_all(&get_vcs_fold())?;
            create_dir_all(&get_vcs_fold())?;
            println!("The '.chak' folder has been reinitialized.");
        } else {
            println!("Operation canceled. The '.chak' folder remains unchanged.");
        }
    } else {
        // Create the .chak folder
        create_dir_all(&get_vcs_fold())?;
        println!(
            "Initialized empty Chak repository in '{}'",
            get_vcs_fold().display()
        );
    }

    create_dir_all(&staging_area_fold())?;
    create_dir_all(&blob_fold())?;
    create_dir_all(&commits_fold())?;
    create_dir_all(&history_fold())?;

    save_or_create_file(&staging_area_fold().join("stage"), None, false)?;
    save_or_create_file(&history_fold().join("commit_log"), None, false)?;

    Ok(())
}
