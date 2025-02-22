
use crate::config::{blob_fold, commits_fold,get_commit_log_file, get_config, get_project_dir, trees_fold, vcs_fold, versions_fold, Config, VCS_FOLDER, VCS_IGNORE_FILE};


use crate::object_pointer::{ObjectPointer};
use crate::util::save_or_create_file;
use crate::util::{deserialize_file_content, serialize_struct};
use crate::util::{read_directory_entries, string_content_to_string_vec};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use ignore::Match;
use indexmap::{IndexMap, IndexSet};
use itertools::{all, Itertools};
use std::collections::HashSet;
use std::fs::File;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::{fs, io};
use crate::hashed_blob::BlobHashPointer;
use crate::commit::{Commit, CommitHashPointer};
use crate::tree_object::{attach_latest_tree_root_pointer_to_stage, TreeHashPointer, TreeObject};
use crate::version_hashed::VersionHashPointer;
use crate::version_head::VersionHeadHashPointer;
use crate::hash_pointer::HashPointerTraits;
use crate::versioning::VersionHead;

pub fn start_snapshot(vcs_config: &Config) -> io::Result<()> {
    //all in one ignore vec that handles multiple ignore file present in nested folder
    let mut main_ignore_builder = GitignoreBuilder::new(get_project_dir());
    let ignore_file = get_project_dir().join(VCS_IGNORE_FILE);
    main_ignore_builder.add(ignore_file);
    main_ignore_builder.add(VCS_FOLDER); //i want to ignore chak folder at start or top ".chak/"

    //implement the tree pointer with traversing fold/file and checking hash from tree pointer and so on .. TODO
    //get latest tree pointer from history_log
    let tree_object = TreeObject::get_top_most_tree_object();

    //here we start taking new updated snapshot of our directory from project root dir, and it gives as the latest updated tree pointer
    let new_root_tree_pointer = dir_snapshot(
        vcs_config,
        get_project_dir(),
        &mut main_ignore_builder,
        tree_object,
    );

    //attaching the updated new tree pointer to stage temporarily because tree pointer can be changed util its commited
    attach_latest_tree_root_pointer_to_stage(new_root_tree_pointer);
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
    tree_hie: Option<TreeObject>,
) -> TreeHashPointer{
    // we cant take dir snapshot if path is file.
    assert!(dir_path.is_dir(), "Path is not a directory");

    // children of this dir maps to their update or TreeNode
    let mut children = TreeObject::new();
    //
    // let ignore_file = path.join(VCS_IGNORE_FILE);
    // main_ignore_builder.add(ignore_file);
    // Handle .ignore file

    if vcs_config.vcs_work_with_nested_ignore_file {
        main_ignore_builder.add(dir_path.join(VCS_IGNORE_FILE));
    } else {
        handle_ignore_file(
            main_ignore_builder,
            vec![(Some(dir_path.to_path_buf()), VCS_IGNORE_FILE)],
        );
    }

    //ignore file would be handled through this functions
    let allowed_entries =
        parse_ignore_local_level(dir_path, main_ignore_builder).unwrap_or_default();

    for allowed_entry in allowed_entries {
        //like file name or folder name not their path addr
        let entry_name = allowed_entry
            .file_name()
            .expect("Could not get file name")
            .to_str()
            .expect("Could not convert to str")
            .to_string();

        let child = process_entry(&allowed_entry, &entry_name,tree_hie.clone(), main_ignore_builder, vcs_config );
            children.add_child(entry_name, child);

    }

    //need to save tree temporary for other process , it has to be saved only when commited with message
    TreeHashPointer::save_tree(&mut children)
}

fn process_entry(
    entry: &Path,
    entry_name: &str,
    tree_hie: Option<TreeObject>,
    main_ignore_builder: &mut GitignoreBuilder,
    vcs_config: &Config,
) -> ObjectPointer {

    let blob_hash_pointer = BlobHashPointer::save_blob_from_file(&entry);

    match child_tree_node_from_tree_object(tree_hie.as_ref(), entry_name.to_string()) {
        None => {
            if entry.is_file() {
                ObjectPointer::VersionHeadFile(VersionHeadHashPointer::save_version_head(&VersionHead::new(blob_hash_pointer, None)))
            }else {
                let new_child_tree_pointer = dir_snapshot(vcs_config, entry, main_ignore_builder, None);
                ObjectPointer::TreeFIle(new_child_tree_pointer)
            }
        }
        Some(existing_version) => {
            match existing_version {
                ObjectPointer::VersionHeadFile(vh) => {
                    let mut version_head = vh.load_version_head();
                    let new_version_head_hash_pointer = version_head.create_version(blob_hash_pointer);
                    ObjectPointer::VersionHeadFile(new_version_head_hash_pointer)

                }
                ObjectPointer::TreeFIle(tree_pointer) => {
                    let existing_child_tree = deserialize_file_content::<TreeObject>(
                        &trees_fold().join(tree_pointer.get_path()),
                    )
                        .expect("Could not deserialize tree");

                    let new_child_tree_pointer = dir_snapshot(vcs_config, entry, main_ignore_builder, Some(existing_child_tree));
                    ObjectPointer::TreeFIle(new_child_tree_pointer)

                }
            }
        }
    }

}


/// Processes a file entry and updates the `children` map


pub fn child_tree_node_from_tree_object(
    tree_hie: Option<&TreeObject>,
    entry_name: String,
) -> Option<ObjectPointer> {
    if let Some(tree) = tree_hie {
        // Borrow instead of moving
        if let Some(child_tree_node) = tree.children.get(&entry_name) {
            println!("entry name exists");
            return Some(child_tree_node.clone()); // Clone if TreeNode needs to be owned
        }
    }

    None
}

pub fn parse_ignore_local_level(
    dir_path: &Path,
    main_ignore_builder: &mut GitignoreBuilder,
) -> io::Result<HashSet<PathBuf>> {
    // Read and filter directory entries
    let detected_entries = read_directory_entries(dir_path)?;
    let mut allowed_entries = HashSet::new();

    if let Ok(build_ignore_rules) = main_ignore_builder.build() {
        // Check entries against ignore rules
        for entry in detected_entries.clone() {
            match build_ignore_rules.matched(entry.to_str().unwrap_or(""), entry.is_dir()) {
                // can i use "#" for default
                Match::None => {
                    allowed_entries.insert(entry.clone());
                }
                Match::Ignore(_) => {
                    if allowed_entries.contains(&entry) {
                        //remove the entry from allowed list
                        allowed_entries.remove(&entry);
                    }
                    println!("Ignored: {}", entry.display());
                }
                Match::Whitelist(_) => {
                    allowed_entries.insert(entry.clone());
                }
            }
        }
    };

    Ok(allowed_entries)
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
