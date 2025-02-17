use crate::commit::{attach_latest_root_pointer_to_stage, Commit};
use crate::config::{get_config, get_project_dir, Config, VCS_FOLDER};
use crate::diff::{hashed_content_from_string_lines, HashedContent};
use crate::diff_algo::{compare_hashed_content, hashed_content_from_path};
use crate::hashing::{
    get_latest_pointer_line_from_file, hash_and_content_from_file_path_ref, hash_from_save_content,
    hash_from_save_tree, HashPointer,
};
use crate::tree_object::{TreeNode, TreeObject, TreeObjectType};
use crate::util::{check_vcs_presence, save_or_create_file};
use crate::util::{deserialize_file_content, serialize_struct};
use crate::util::{read_directory_entries, string_content_to_string_vec};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use ignore::Match;
use indexmap::{IndexMap, IndexSet};
use itertools::{all, Itertools};
use std::fs::File;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::{fs, io};
use std::collections::HashSet;

pub fn start_snapshot(vcs_config: &Config, project_dir: &Path) -> io::Result<()> {
    //all in one ignore vec that handles multiple ignore file present in nested folder
    let mut main_ignore_builder = GitignoreBuilder::new(project_dir);
    let ignore_file = project_dir.join(&vcs_config.vcs_ignore_file);
    main_ignore_builder.add(ignore_file);
    main_ignore_builder.add(VCS_FOLDER);//i want to ignore chak folder at start or top ".chak/"

    //implement the tree pointer with traversing fold/file and checking hash from tree pointer and so on .. TODO
    //get latest tree pointer from history_log
    let mut tree_object: Option<TreeObject> = None;

    // as commit log file created at initialization
    let commit_file =File::open(&vcs_config.get_commit_log()).expect("Unable to open commit_log file");

    if let Some(commit_pointer) = get_latest_pointer_line_from_file(&commit_file, true) {
        if let Ok(latest_commit) =
            deserialize_file_content::<Commit>(&vcs_config.commits_fold().join(commit_pointer.get_path()))
        //getting previous commit that was saved in commit fold
        {
            // from commit ,getting pointer to previous tree structure that represent the file/folder hierarchy
            let root_tree_pointer = latest_commit.root_tree_pointer.get_path();

            //fetching latest tree from trees fold and converting it to TreeObject so that we can use in our program
            match deserialize_file_content::<TreeObject>(&vcs_config.trees_fold().join(root_tree_pointer)) {
                Ok(tree_object_s) => {
                    tree_object = Some(tree_object_s);
                    println!("We got tree_object");
                }
                Err(e) => {
                    eprintln!("We got an error: {}", e);
                }
            }
        }
    }

    //here we start taking new updated snapshot of our directory from project root dir, and it gives as the latest updated tree pointer
    let new_root_tree_pointer = dir_snapshot(vcs_config, project_dir,  &mut main_ignore_builder, tree_object)?;

    //attaching the updated new tree pointer to stage temporarily because tree pointer can be changed util its commited
    attach_latest_root_pointer_to_stage(new_root_tree_pointer);
    Ok(())
}

/// Handles .ignore file processing and adds it to `ignore_build_vec`
fn handle_ignore_file(
    main_ignore_builder: &mut GitignoreBuilder,
    ignore_these_also: Vec<(Option<PathBuf>, &str)>,
) {
    if ! ignore_these_also.is_empty() {
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
    path: &Path,
    main_ignore_builder: &mut GitignoreBuilder,
    tree_hie: Option<TreeObject>,
) -> io::Result<HashPointer> {
    // we cant take dir snapshot if path is file.
    assert!(path.is_dir(), "Path is not a directory");

    // children of this dir maps to their update or TreeNode
    let mut children = IndexMap::<String, TreeNode>::new();
    //
    // let ignore_file = path.join(VCS_IGNORE_FILE);
    // main_ignore_builder.add(ignore_file);
    // Handle .ignore file
    handle_ignore_file( main_ignore_builder,vec![  (Some(path.to_path_buf()), &vcs_config.vcs_ignore_file) ] );

    //ignore file would be handled through this functions
    let allowed_entries_no_ignore_files =
        parse_ignore_local_level(path, main_ignore_builder).unwrap_or_default();

    for allowed_entry in allowed_entries_no_ignore_files {
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

        }else { println!("Could not process entry: {}", allowed_entry.display()); };
    }

    //need to save tree temporary for other process , it has to be saved only when commited with message
    hash_from_save_tree(&vcs_config.trees_fold(), children)
}

/// Processes a directory entry and updates the `children` map
fn process_directory(
    entry: &Path,
    entry_name: &str,
    tree_hie: Option<TreeObject>,
    main_ignore_builder: &mut GitignoreBuilder,
    vcs_config: &Config,
) -> io::Result<TreeNode> {
    let existing_tree = child_tree_node_from_tree_object(tree_hie.as_ref(), entry_name.to_string())
        .and_then(|obj| {
            if obj.node_type == TreeObjectType::TreeObject {
                deserialize_file_content::<TreeObject>(
                    &vcs_config.trees_fold().join(obj.hash_pointer_to_this_node.get_path()),
                )
                .ok()
            } else {
                None
            }
        });

    let new_tree_hash = dir_snapshot(vcs_config, entry, main_ignore_builder, existing_tree)?;

    Ok(TreeNode {
        node_type: TreeObjectType::TreeObject,
        hash_pointer_to_this_node: new_tree_hash,
        hash_pointer_to_diff: None,
    })
}

fn process_file(
    vcs_config: &Config,
    entry: &Path,
    entry_name: &str,
    tree_hie: Option<TreeObject>,
) -> io::Result<TreeNode> {
    let (new_file_hash, new_file_content) = hash_and_content_from_file_path_ref(&entry)?;

    if !vcs_config.blob_fold().join(new_file_hash.get_path()).exists() {
        save_or_create_file(
            &vcs_config.blob_fold().join(&new_file_hash.get_path()),
            Some(&new_file_content),
            false,
            None,
        )
        .expect("Could not save file");
    }

    if let Some(mut existing_version) =
        child_tree_node_from_tree_object(tree_hie.as_ref(), entry_name.to_string())
    {
        process_file_when_previous_version_exist(
            vcs_config,
            &new_file_hash,
            &hashed_content_from_string_lines(string_content_to_string_vec(&new_file_content)),
            &mut existing_version,
        )
    } else {
        Ok(TreeNode {
            node_type: TreeObjectType::BlobFile,
            hash_pointer_to_this_node: new_file_hash,
            hash_pointer_to_diff: None,
        })
    }
}
/// Processes a file entry and updates the `children` map
fn process_file_when_previous_version_exist(
    vcs_config: &Config,
    new_file_hash: &HashPointer,
    new_file_hashed_content: &HashedContent,
    existing_version: &mut TreeNode,
) -> io::Result<TreeNode> {
    if new_file_hash != &existing_version.hash_pointer_to_this_node {
        let previous_blob_path =
            &vcs_config.blob_fold().join(existing_version.hash_pointer_to_this_node.get_path());
        let prev_blob_hashed_content = hashed_content_from_path(&previous_blob_path);

        let mut diff = compare_hashed_content(&prev_blob_hashed_content, new_file_hashed_content);
        if let Some(prev_version) = existing_version.hash_pointer_to_diff.clone() {
            diff.pointer_to_previous_version = Some(prev_version);
        }

        let diff_hash = hash_from_save_content(&serialize_struct(&diff), &vcs_config.versions_fold())?; // diff has to be saved in version fold
        existing_version.hash_pointer_to_diff = Some(diff_hash);

        // Remove old blob
        if let Err(e) = fs::remove_file(previous_blob_path) {
            eprintln!("Warning: Failed to delete file: {}", e);
        }
        existing_version.hash_pointer_to_this_node = new_file_hash.clone();
    } else {
        eprintln!("Warning: File hash already exists ; no need extra effort ");
    }

    Ok(existing_version.clone())
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
                Match::None => {
                    allowed_entries.insert(entry.clone());
                },
                Match::Ignore(_) => {
                    if allowed_entries.contains(&entry) {
                        //remove the entry from allowed list
                        allowed_entries.remove(&entry);
                    }
                    println!("Ignored: {}", entry.display());
                },
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
    if check_vcs_presence(get_project_dir()) {
        if files.contains(&".".to_string()) { //i have to fix this in future check for . in first string
            start_snapshot(&config, get_project_dir()).expect("cant start the snapshot");
        }
    } else {
        println!("No vcs_presence configured. could not applied add operations.");
    }
}