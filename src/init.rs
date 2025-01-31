use std::{fs};

use crate::config::{blob_fold, commit_history_fold, commits_fold, get_current_dir, get_vcs_fold , staging_area_fold};
use crate::macros::{create_file, create_fold, input_from_commandline};

pub fn init() {

    if get_vcs_fold().exists() {
        let choice = input_from_commandline("A '.chak' folder already exists in this directory. Do you want to override it? (y/n) ");

        if choice == "y" {
            // Delete existing .chak folder and recreate it
            if let Err(e) = fs::remove_dir_all(&get_vcs_fold()) {
                eprintln!("Error removing existing '.chak' folder: {}", e);
                return;
            }
            create_fold(&get_vcs_fold()                                                                                                                        );
            println!("The '.chak' folder has been reinitialized.");
        } else {
            println!("Operation canceled. The '.chak' folder remains unchanged.");
        }
    } else {
        // Create the .chak folder
        create_fold(&get_vcs_fold());
        println!("Initialized empty Chak repository in '{}'", get_vcs_fold().display());
    }


    create_fold(&staging_area_fold());
    create_fold(&blob_fold());
    create_fold(&commits_fold());
    create_fold(&commit_history_fold());

    create_file(&staging_area_fold().join("stage"));
    create_file(&commit_history_fold().join("commit_log"))


    // create_fold(&store_path.join("packs"));

}

