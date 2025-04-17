use crate::config::{
    CHAK_FOLDER_NAME, Config, VCS_IGNORE_FILE_NAME, get_chak_fold_path, get_config,
    get_current_dir_path,
};
use crate::custom_error::ChakError;
use crate::handle_ignore::{handle_ignore_file, parse_ignore_combined_files_dirs};
use crate::root_tree_object::{NestedTreeObject, RootTreeObject};
use crate::root_tree_pointer::RootTreePointer;
use crate::takesnapshot::start_individual_snapshot;
use ignore::gitignore::GitignoreBuilder;
use std::io;
use std::path::Path;

pub fn start_snapshot(vcs_config: &Config) -> Result<(), ChakError> {
    //all in one ignore vec that handles multiple ignore file present in nested folder
    let mut main_ignore_builder = GitignoreBuilder::new(get_current_dir_path());
    let ignore_file = get_current_dir_path().join(VCS_IGNORE_FILE_NAME);
    main_ignore_builder.add(ignore_file); // there is no need to ignore ingorefile

    handle_ignore_file(
        &mut main_ignore_builder,
        vec![(None, CHAK_FOLDER_NAME)],
    );

    main_ignore_builder.add(get_chak_fold_path()); //i want to ignore chak folder at start or top ".chak/"

    //implement the tree pointer with traversing fold/file and checking hash from tree pointer and so on .. TODO
    //get latest tree pointer from history_log
    let root_tree = RootTreeObject::get_root_object().unwrap_or(RootTreeObject::new());
    let mut as_nested_tree = root_tree.as_nested_tree();

    //here we start taking new updated snapshot of our directory from project root dir, and it gives as the latest updated tree pointer
    dir_snapshot(
        vcs_config,
        get_current_dir_path(),
        &mut main_ignore_builder,
        &mut as_nested_tree, // this as nested creating new clone of root disconnected one
    )?;

    let new_root_tree_pointer =
        RootTreePointer::save_tree(&mut RootTreeObject::from(as_nested_tree))?;
    //attaching the updated new tree pointer to stage temporarily because tree pointer can be changed util its commited
    new_root_tree_pointer.attach_tree_to_stage();
    Ok(())
}

pub fn dir_snapshot(
    vcs_config: &Config,
    dir_path: &Path,
    main_ignore_builder: &mut GitignoreBuilder,
    tree_ref: &mut NestedTreeObject,
) -> Result<(), ChakError> {
    if !dir_path.is_dir() {
        return Err(ChakError::CustomError(
            io::ErrorKind::NotADirectory.to_string(),
        ));
    }

    if vcs_config.vcs_work_with_nested_ignore_file {
        main_ignore_builder.add(dir_path.join(VCS_IGNORE_FILE_NAME));
    } else {
        handle_ignore_file(
            main_ignore_builder,
            vec![(Some(dir_path.to_path_buf()), VCS_IGNORE_FILE_NAME)],
        );
    }

    let allowed_entries = parse_ignore_combined_files_dirs(dir_path, main_ignore_builder)?;

    for entry in allowed_entries {
        let entry_name = entry
            .file_name()
            .expect("Could not get file name")
            .to_str()
            .expect("Could not convert to str")
            .to_string();

        if entry.is_file() {
            tree_ref.add_file_child(&entry, &entry_name)?;
        } else {
            if let Some(existing_child_tree) = tree_ref.dir_children.get_mut(&entry_name) {
                dir_snapshot(
                    vcs_config,
                    &entry,
                    main_ignore_builder,
                    &mut existing_child_tree.load_tree(),
                )?;
            } else {
                //making sure that nested tree object so that we can procede with nested dir
                let mut new_dir_nested_tree_object = NestedTreeObject::new();

                dir_snapshot(
                    vcs_config,
                    &entry,
                    main_ignore_builder,
                    &mut new_dir_nested_tree_object,
                )?;

                tree_ref.add_dir_child(entry_name, &mut new_dir_nested_tree_object)?;
            }
        }
    }

    Ok(())
}

pub fn command_add(files: Vec<String>) -> Result<(), ChakError> {
    let config = get_config();

    if get_chak_fold_path().exists() && get_chak_fold_path().is_dir() {
        if files.contains(&".".to_string()) {
            //i have to fix this in future check for . in first string
            start_snapshot(&config)?;
        } else {
            for file in files {
                println!("adding {}", file);
                start_individual_snapshot(file)?;
            }
        }
    } else {
        return Err(ChakError::RepoNotInitialized);
    }

    Ok(())
}
