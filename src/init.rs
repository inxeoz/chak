use std::{fs};

use crate::config::{get_current_dir};
use crate::config::VCS_FOLDER;
use crate::macros::{create_fold, input_from_commandline};

pub fn init() {


    let vcs_path =&get_current_dir().join(VCS_FOLDER);

    if vcs_path.exists() {
        let choice = input_from_commandline("A '.chak' folder already exists in this directory. Do you want to override it? (y/n) ");

        if choice == "y" {
            // Delete existing .chak folder and recreate it
            if let Err(e) = fs::remove_dir_all(&vcs_path) {
                eprintln!("Error removing existing '.chak' folder: {}", e);
                return;
            }
            create_fold(&vcs_path                                                                                                                        );
            println!("The '.chak' folder has been reinitialized.");
        } else {
            println!("Operation canceled. The '.chak' folder remains unchanged.");
        }
    } else {
        // Create the .chak folder
        create_fold(&vcs_path);
        println!("Initialized empty Chak repository in '{}'", vcs_path.display());
    }
        //commandline -> in this folder .chak folder exist previoulsy , do you want to override ? y/n
        //if input y then createfold!(&vcs_path); else return


    let store_path = vcs_path.join("store");
    create_fold(&store_path);

    create_fold(&store_path.join("blobs"));
    create_fold(&store_path.join("trees"));
    create_fold(&store_path.join("commits"));
    create_fold(&store_path.join("packs"));

}

