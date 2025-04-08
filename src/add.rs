use crate::config::{Config, VCS_FOLDER, VCS_IGNORE_FILE, get_config, get_current_dir, vcs_fold};
use crate::custom_error::ChakError;
use crate::handle_ignore::{handle_ignore_file, parse_ignore};
use crate::root_tree_object::{NestedTreeObject, RootTreeObject};
use crate::root_tree_pointer::RootTreePointer;
use crate::takesnapshot::{start_individual_snapshot, take_snapshot};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::{Path, PathBuf};

pub fn start_snapshot(vcs_config: &Config) -> Result<(), ChakError> {
    //all in one ignore vec that handles multiple ignore file present in nested folder
    let mut main_ignore_builder = GitignoreBuilder::new(get_current_dir());
    let ignore_file = get_current_dir().join(VCS_IGNORE_FILE);
    main_ignore_builder.add(ignore_file); // there is no need to ignore ingorefile
    main_ignore_builder.add(VCS_FOLDER); //i want to ignore chak folder at start or top ".chak/"

    //implement the tree pointer with traversing fold/file and checking hash from tree pointer and so on .. TODO
    //get latest tree pointer from history_log
    let root_tree = RootTreeObject::get_root_object().unwrap_or(RootTreeObject::new());
    let mut as_nested_tree = root_tree.as_nested_tree();

    //here we start taking new updated snapshot of our directory from project root dir, and it gives as the latest updated tree pointer
    dir_snapshot(
        vcs_config,
        get_current_dir(),
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
    assert!(dir_path.is_dir(), "Path is not a directory"); //check condition or panic

    if vcs_config.vcs_work_with_nested_ignore_file {
        main_ignore_builder.add(dir_path.join(VCS_IGNORE_FILE));
    } else {
        handle_ignore_file(
            main_ignore_builder,
            vec![(Some(dir_path.to_path_buf()), VCS_IGNORE_FILE)],
        );
    }

    let allowed_entries = parse_ignore(dir_path, main_ignore_builder).map(|(mut v1, v2)| {
        v1.extend(v2);
        return v1;
    })?;

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

pub fn command_add(files: Vec<String>) {
    let config = get_config();

    if vcs_fold().exists() && vcs_fold().is_dir() {
        if files.contains(&".".to_string()) {
            //i have to fix this in future check for . in first string
            start_snapshot(&config).expect("cant start the snapshot");
        } else {
            for file in files {
                println!("adding {}", file);
                start_individual_snapshot(&get_config(), file).expect("TODO: panic message");
            }
        }
    } else {
        println!("No vcs_presence configured. could not applied add operations.");
    }
}
