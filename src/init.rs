use crate::config::{essentials_files, essentials_folds, get_config_file, save_config, vcs_fold, Config};
use crate::util::{input_from_commandline, save_or_create_file};
use std::fs::create_dir_all;
use std::{fs, io};
use crate::global_config::get_global_config;

pub fn init() -> Result<(), io::Error> {
    if vcs_fold().exists() {
        // checking .chak/ folder exist or not

        let choice = input_from_commandline(
            "A '.chak' folder already exists in this directory. Do you want to override it? (y/n) ",
        )
        .unwrap_or("n".to_string());

        if choice.to_lowercase() != "y" {
            println!("Operation canceled. The '.chak' folder remains unchanged.");
            return Ok(());
        } else {
            fs::remove_dir_all(vcs_fold())?; // removing previously exist .chak/ folder
            println!("Initializing The '.chak' folder");
        }
    } else {
        println!(
            "Initializing empty Chak repository in '{}'",
            vcs_fold().display()
        );
    }

    initialize_vcs();
    println!("done! vcs exist now");

    Ok(())
}

pub fn initialize_vcs() {
    for essentials_fold in essentials_folds() {
        create_dir_all(essentials_fold).expect("Failed to create VCS folders");
    }
    for essentials_file in essentials_files() {
        save_or_create_file(&essentials_file, None, false, None)
            .expect("Failed to create vcs files");
    }

    let global_config = get_global_config();
    let new_config = Config::new(&global_config);
    save_config(&new_config)

}
