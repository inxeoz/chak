use std::collections::HashSet;
use std::io;
use std::path::{Path, PathBuf};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use ignore::Match;
use crate::config::{get_config, get_project_dir, vcs_fold, Config, VCS_FOLDER, VCS_IGNORE_FILE};
use crate::blob_hash_pointer::BlobHashPointer;
use crate::custom_error::ChakError;
use crate::tree_hash_pointer::{ TreeHashPointer};
use crate::tree_object::TreeObject;
use crate::util::read_directory_entries;
use crate::version_head::VersionHeadHashPointer;
use crate::versioning::VersionHead;

pub fn start_snapshot(vcs_config: &Config) -> Result<(), ChakError> {
    //all in one ignore vec that handles multiple ignore file present in nested folder
    let mut main_ignore_builder = GitignoreBuilder::new(get_project_dir());
    let ignore_file = get_project_dir().join(VCS_IGNORE_FILE);
    main_ignore_builder.add(ignore_file);
    main_ignore_builder.add(VCS_FOLDER); //i want to ignore chak folder at start or top ".chak/"

    //implement the tree pointer with traversing fold/file and checking hash from tree pointer and so on .. TODO
    //get latest tree pointer from history_log
    let mut tree_object = TreeObject::get_top_most_tree_object().unwrap_or(TreeObject::new());

    //here we start taking new updated snapshot of our directory from project root dir, and it gives as the latest updated tree pointer
    dir_snapshot(
        vcs_config,
        get_project_dir(),
        &mut main_ignore_builder,
        &mut tree_object,
    );

    let new_root_tree_pointer = TreeHashPointer::save_tree(&mut tree_object);
    //attaching the updated new tree pointer to stage temporarily because tree pointer can be changed util its commited
    new_root_tree_pointer.attach_tree_to_stage();
    Ok(())
}

/// Handles .ignore file processing and adds it to `ignore_build_vec`
fn handle_ignore_file(
    main_ignore_builder: &mut GitignoreBuilder,
    ignore_these_also: Vec<(Option<PathBuf>, &str)>,
) {
    if !ignore_these_also.is_empty() {
        // Add extra ignore rules. Handle errors gracefully.
        for (base_dir, ignore_this) in ignore_these_also {
            if let Err(err) = main_ignore_builder.add_line(base_dir, ignore_this) {
                eprintln!("Error adding ignore rule '{}': {}", ignore_this, err);
                // We could choose to continue here even with a bad rule
            }
        }
    }
}

pub fn dir_snapshot(
    vcs_config: &Config,
    dir_path: &Path,
    main_ignore_builder: &mut GitignoreBuilder,
    tree_ref: &mut TreeObject,
) {
    assert!(dir_path.is_dir(), "Path is not a directory");

    if vcs_config.vcs_work_with_nested_ignore_file {
        main_ignore_builder.add(dir_path.join(VCS_IGNORE_FILE));
    } else {
        handle_ignore_file(
            main_ignore_builder,
            vec![(Some(dir_path.to_path_buf()), VCS_IGNORE_FILE)],
        );
    }

    let allowed_entries = parse_ignore(dir_path, main_ignore_builder).unwrap_or_default();

    for entry in allowed_entries
    {
        let entry_name = entry
            .file_name()
            .expect("Could not get file name")
            .to_str()
            .expect("Could not convert to str")
            .to_string();

        if entry.is_file() {
            process_file_entry(&entry, &entry_name, tree_ref);
        } else {
            if ! tree_ref.dir_children.contains_key(&entry_name) {
                tree_ref.add_dir_child(entry_name.clone(), &mut TreeObject::new());
            }
            if let Some( existing_child_tree) =tree_ref.dir_children.get_mut(&entry_name) {
                dir_snapshot(
                    vcs_config,
                    &entry,
                    main_ignore_builder,
                    &mut existing_child_tree.load_tree(),
                );
            }
        }
    }
}

fn process_file_entry(file_entry: &Path, entry_name: &str, tree_ref: &mut TreeObject) {
    let blob_hash_pointer = BlobHashPointer::save_blob_from_file(&file_entry);
    //create list to add this tempo
    if let Some(existing_version) = tree_ref.file_children.get(&entry_name.to_string()) {
        let mut version_head = existing_version.load_version_head();
        let updated_version_head_hash_pointer =
            version_head.create_version(blob_hash_pointer.clone());
        tree_ref.add_file_child(entry_name.to_string(), updated_version_head_hash_pointer);
    } else {
        let new_version_head_hash_pointer =
            VersionHeadHashPointer::save_version_head(&VersionHead::new(blob_hash_pointer, None));
        tree_ref.add_file_child(entry_name.to_string(), new_version_head_hash_pointer);
    }
}
pub fn parse_ignore(
    dir_path: &Path,
    ignore_builder: &mut GitignoreBuilder,
) -> Result< Vec<PathBuf>, ChakError > {
    // Read and filter directory entries
    let (mut detected_dir_entries, mut detected_file_entries) = read_directory_entries(dir_path)?;

    let mut allowed_dir_entries = Vec::new();
    let mut allowed_file_entries = Vec::new();
    if let Ok(build_ignore_rules) = ignore_builder.build() {
        allowed_dir_entries =
            parse_ignore_for_entries(&mut detected_dir_entries, &build_ignore_rules);
        allowed_file_entries =
            parse_ignore_for_entries(&mut detected_file_entries, &build_ignore_rules);
    }

    allowed_file_entries.extend(allowed_dir_entries);
    Ok(allowed_file_entries)
}

pub fn parse_ignore_for_entries(
    detected_entries: &mut Vec<PathBuf>,
    ignore_build: &Gitignore,
) -> Vec<PathBuf> {
    let mut allowed_entries = HashSet::new();

    for entry in detected_entries {
        match ignore_build.matched(entry.to_str().unwrap_or(""), entry.is_dir()) {
            // can i use "#" for default
            Match::None => {
                allowed_entries.insert(entry.clone());
            }
            Match::Ignore(_) => {
                if allowed_entries.contains(entry.as_path()) {
                    allowed_entries.remove(&entry.clone());
                }
                println!("Ignored: {}", entry.display());
            }
            Match::Whitelist(_) => {
                allowed_entries.insert(entry.clone());
            }
        }
    }
    allowed_entries.into_iter().collect()
}
pub fn command_add(files: Vec<String>) {
    let config = get_config();

    if vcs_fold().exists() && vcs_fold().is_dir() {
        if files.contains(&".".to_string()) {
            //i have to fix this in future check for . in first string
            start_snapshot(&config).expect("cant start the snapshot");
        }
    } else {
        println!("No vcs_presence configured. could not applied add operations.");
    }
}
