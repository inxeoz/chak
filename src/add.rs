use crate::commit::{attach_latest_root_pointer_to_stage, Commit};
use crate::config::{
    blob_fold, commits_fold, get_commit_log, get_config, get_project_dir, trees_fold, vcs_fold,
    versions_fold, Config, VCS_FOLDER, VCS_IGNORE_FILE,
};
use crate::diff::{hashed_content_from_string_lines, HashedContent, HashedContentForVersion};
use crate::diff_algo::{compare_hashed_content, hashed_content_from_path};
use crate::hashing::{
    get_latest_pointer_line_from_file, hash_and_content_from_file_path_ref, hash_from_save_content,
    hash_from_save_tree, HashPointer,
};
use crate::tree_object::{TreeNode, TreeObject, TreeObjectType};
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

pub fn start_snapshot(vcs_config: &Config) -> io::Result<()> {
    //all in one ignore vec that handles multiple ignore file present in nested folder
    let mut main_ignore_builder = GitignoreBuilder::new(get_project_dir());
    let ignore_file = get_project_dir().join(VCS_IGNORE_FILE);
    main_ignore_builder.add(ignore_file);
    main_ignore_builder.add(VCS_FOLDER); //i want to ignore chak folder at start or top ".chak/"

    //implement the tree pointer with traversing fold/file and checking hash from tree pointer and so on .. TODO
    //get latest tree pointer from history_log
    let mut tree_object: Option<TreeObject> = None;

    // as commit log file created at initialization
    let commit_file = File::open(get_commit_log()).expect("Unable to open commit_log file");

    if let Some(commit_pointer) = get_latest_pointer_line_from_file(&commit_file, true) {
        if let Ok(latest_commit) =
            deserialize_file_content::<Commit>(&commits_fold().join(commit_pointer.get_path()))
        //getting previous commit that was saved in commit fold
        {
            // from commit ,getting pointer to previous tree structure that represent the file/folder hierarchy
            let root_tree_pointer = latest_commit.root_tree_pointer.get_path();

            //fetching latest tree from trees fold and converting it to TreeObject so that we can use in our program
            match deserialize_file_content::<TreeObject>(&trees_fold().join(root_tree_pointer)) {
                Ok(tree_object_s) => {
                    tree_object = Some(tree_object_s);
                    println!("We got tree_object");
                }
                Err(e) => {
                    eprintln!("there is no root tree from commit  {}", e);
                }
            }
        }
    }

    //here we start taking new updated snapshot of our directory from project root dir, and it gives as the latest updated tree pointer
    let new_root_tree_pointer = dir_snapshot(
        vcs_config,
        get_project_dir(),
        &mut main_ignore_builder,
        tree_object,
    )?;

    //attaching the updated new tree pointer to stage temporarily because tree pointer can be changed util its commited
    attach_latest_root_pointer_to_stage(new_root_tree_pointer);
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
) -> io::Result<HashPointer> {
    // we cant take dir snapshot if path is file.
    assert!(dir_path.is_dir(), "Path is not a directory");

    // children of this dir maps to their update or TreeNode
    let mut children = IndexMap::<String, TreeNode>::new();
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

        if let Ok(child) = if allowed_entry.is_file() {
            process_file(vcs_config, &allowed_entry, &entry_name, tree_hie.clone())
        } else {
            process_directory(
                &allowed_entry,
                &entry_name,
                tree_hie.clone(),
                main_ignore_builder,
                vcs_config,
            )
        } {
            children.insert(entry_name.to_string(), child);
        } else {
            println!("Could not process entry: {}", allowed_entry.display());
        };
    }

    //need to save tree temporary for other process , it has to be saved only when commited with message
    hash_from_save_tree(&trees_fold(), children)
}

/// Processes a directory entry and updates the `children` map
fn process_directory(
    entry: &Path,
    entry_name: &str,
    tree_hie: Option<TreeObject>,
    main_ignore_builder: &mut GitignoreBuilder,
    vcs_config: &Config,
) -> io::Result<TreeNode> {
    let existing_child_tree =
        child_tree_node_from_tree_object(tree_hie.as_ref(), entry_name.to_string()).and_then(
            |obj| {
                if obj.node_type == TreeObjectType::TreeObject {
                    deserialize_file_content::<TreeObject>(
                        &trees_fold().join(obj.hash_pointer_to_this_node.get_path()),
                    )
                    .ok()
                } else {
                    None
                }
            },
        );

    let new_child_tree_pointer =
        dir_snapshot(vcs_config, entry, main_ignore_builder, existing_child_tree)?;

    Ok(TreeNode {
        node_type: TreeObjectType::TreeObject,
        hash_pointer_to_this_node: new_child_tree_pointer,
        hash_pointer_to_previous_version: None,
    })
}

fn process_file(
    vcs_config: &Config,
    entry: &Path,
    entry_name: &str,
    tree_hie: Option<TreeObject>,
) -> io::Result<TreeNode> {
    let (new_file_hash, new_file_content) = hash_and_content_from_file_path_ref(&entry)?;

    if !blob_fold().join(new_file_hash.get_path()).exists() {
        save_or_create_file(
            &blob_fold().join(&new_file_hash.get_path()),
            Some(&new_file_content),
            false,
            None,
        )
        .expect("Could not save file");
    }

    if let Some(mut existing_version) =
        child_tree_node_from_tree_object(tree_hie.as_ref(), entry_name.to_string())
    {
        // if new_file_hash != existing_version.hash_pointer_to_this_node , this hash checking was done blob_fold().join(new_file_hash.get_path()).exists() up there
        process_file_when_previous_version_exist(
            &hashed_content_from_string_lines(string_content_to_string_vec(&new_file_content)),
            &mut existing_version,
        ).expect("failed to process existing version "); // this will change some parameter of existing version because i send it &mut ref but it will not consume it,and current scope has ownership

        existing_version.hash_pointer_to_this_node = new_file_hash.clone();
        Ok(existing_version)
    } else {
        Ok(TreeNode {
            node_type: TreeObjectType::BlobFile,
            hash_pointer_to_this_node: new_file_hash,
            hash_pointer_to_previous_version: None,
        })
    }
}
/// Processes a file entry and updates the `children` map
fn process_file_when_previous_version_exist(
    new_file_hashed_content: &HashedContent,
    existing_version: &mut TreeNode,
) -> Result<(), io::Error> {
    let previous_blob_path =
        blob_fold().join(existing_version.hash_pointer_to_this_node.get_path());
    let prev_blob_hashed_content = hashed_content_from_path(&previous_blob_path);

    let mut diff = compare_hashed_content(&prev_blob_hashed_content, new_file_hashed_content);

    let mut diff_content = HashedContentForVersion::new(diff, None);
    if let Some(prev_version) = existing_version.hash_pointer_to_previous_version.clone() {
        diff_content.pointer_to_previous_version = Some(prev_version);
    }

    let diff_hash = hash_from_save_content(&serialize_struct(&diff_content), &versions_fold())?; // diff has to be saved in version fold
    existing_version.hash_pointer_to_previous_version = Some(diff_hash);

    // Remove old blob
    fs::remove_file(previous_blob_path)?;
    Ok(())
}

pub fn child_tree_node_from_tree_object(
    tree_hie: Option<&TreeObject>,
    entry_name: String,
) -> Option<TreeNode> {
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
