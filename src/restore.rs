use crate::config::get_project_dir;
use crate::util::save_or_create_file;
use std::fs::create_dir_all;
use std::io;
use std::path::PathBuf;
use crate::custom_error::ChakError;
use crate::tree_hash_pointer::TreeHashPointer;
use crate::tree_object::TreeObject;

fn start_restoring(tree_object: TreeObject, dir_path: &PathBuf) -> Result<(), ChakError> {
    for (child_name, version_head_pointer) in tree_object.file_children {
        let actual_child_file_path = dir_path.join(PathBuf::from(child_name)); //in working folder

        let hashed_content = version_head_pointer
            .load_version_head()
            .get_pointer_to_blob()
            .load_blob();
        let content = hashed_content.to_string_content();
        //save blob data into actual child
        save_or_create_file(&actual_child_file_path, Some(&content), false, None)?;
    }

    for (child_name, existing_tree_object) in tree_object.dir_children {
        let actual_child_fold_path = dir_path.join(PathBuf::from(child_name)); //in working folder
        if ! actual_child_fold_path.exists() {
            create_dir_all(actual_child_fold_path.clone())?;
        }
        start_restoring(existing_tree_object.clone(), &actual_child_fold_path)?;
    }

    Ok(())
}

pub fn command_restore(files: Vec<String>) {
    if files.contains(&".".to_string()) {
        match TreeHashPointer::get_latest_pointer_from_stage() {
            Ok(latest_tree_pointer) => {
                start_restoring(latest_tree_pointer.load_tree(), get_project_dir())
                    .expect("Failed to start restoring.");
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}
