use crate::config::get_current_dir_path;
use crate::util::save_or_create_file;
use std::fs::create_dir_all;
use std::path::PathBuf;
use crate::custom_error::ChakError;
use crate::nested_tree_object::NestedTreeObject;
use crate::root_tree_pointer::RootTreePointer;

fn start_restoring(tree_object: NestedTreeObject, dir_path: &PathBuf) -> Result<(), ChakError> {
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
        start_restoring(existing_tree_object.load_tree().clone(), &actual_child_fold_path)?;
    }

    Ok(())
}

pub fn handle_command_restore(files: Vec<String>) {
    if files.contains(&".".to_string()) {
        match RootTreePointer::get_latest_pointer_from_stage() {
            Ok(latest_tree_pointer) => {
                start_restoring(latest_tree_pointer.load_tree().as_nested_tree(), get_current_dir_path())
                    .expect("Failed to start restoring.");
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}
