use crate::commit::{attach_latest_root_pointer_to_stage, Commit};
use crate::config::{blob_fold, commits_fold, get_project_dir, history_fold, staging_area_fold, trees_fold, versions_fold, VCS_IGNORE_FILE};
use crate::custom_error::ChakError;
use crate::diff::{hashed_content_from_string_lines, HashedContent};
use crate::diff_algo::{compare_hashed_content, deserialize_file_content, get_diff, hashed_content_from_path, serialize_struct};
use crate::hashing::{
    get_latest_pointer_from_file, hash_and_content_from_file_path_ref, hash_from_file,
    hash_from_save_content, hash_from_save_tree, HashPointer,
};
use crate::tree_object::{TreeNode, TreeObject, TreeObjectType};
use crate::util::{file_to_string, save_or_create_file};
use crate::util::{read_directory_entries, string_content_to_string_vec};
use clap::Error;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use ignore::Match;
use indexmap::{map, IndexMap};
use itertools::Itertools;
use std::fs::File;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn start_snapshot() -> io::Result<()> {
    let mut ignore_build_vec = Vec::<Gitignore>::new();
    let start_path = get_project_dir();

    //implement the tree pointer with traversing fold/file and checking hash from tree pointer and so on .. TODO
    //get latest tree pointer from history_log
    let mut tree_object: Option<TreeObject> = None;
    let commit_file =
        File::open(&history_fold().join("commit_log")).expect("Unable to open commit_log file");

    if let Some(commit_pointer) = get_latest_pointer_from_file(&commit_file, true) {
        if let Ok(latest_commit) =
            deserialize_file_content::<Commit>(&commits_fold().join(commit_pointer.get_path()))
        {
            let root_tree_pointer = latest_commit.root_tree_pointer.get_path();

            match deserialize_file_content::<TreeObject>(&trees_fold().join(root_tree_pointer)) {
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

    let root_tree_pointer = dir_snapshot(start_path, &mut ignore_build_vec, tree_object)?;
    attach_latest_root_pointer_to_stage(root_tree_pointer);
    Ok(())
}

/// Handles .ignore file processing and adds it to `ignore_build_vec`
fn handle_ignore_file(path: &Path, ignore_build_vec: &mut Vec<Gitignore>) {
    let ignore_file = path.join(VCS_IGNORE_FILE);
    if ignore_file.exists() {
        let mut local_builder = GitignoreBuilder::new(path);
        local_builder.add(&ignore_file);
        if let Ok(ignore) = local_builder.build() {
            ignore_build_vec.push(ignore);
        } else {
            eprintln!(
                "Could not create Gitignore from path: {}",
                ignore_file.display()
            );
        }
    }
}

pub fn dir_snapshot(
    path: &Path,
    ignore_build_vec: &mut Vec<Gitignore>,
    tree_hie: Option<TreeObject>,
) -> io::Result<HashPointer> {

    assert!(path.is_dir(), "Path is not a directory");

    let mut children = IndexMap::<String, TreeNode>::new();

    // Handle .ignore file
    handle_ignore_file(path, ignore_build_vec);

    let allowed_entries = parse_ignore_local_level(path, ignore_build_vec);

    for entry in allowed_entries {
        let entry_name = entry
            .file_name()
            .expect("Could not get file name")
            .to_str()
            .expect("Could not convert to str")
            .to_string();

        // Skip `.chak` directory at the project root
        if path == get_project_dir() && entry.ends_with(".chak") {
            continue;
        }



        if entry.is_file()  {
            if &entry_name != VCS_IGNORE_FILE {
                process_file(&entry, &entry_name, tree_hie.clone(), &mut children)?
            }
        } else {
           // let existing_tree = tree_node_from_tree_object(tree_hie.as_ref(), entry_name.to_string())
            process_directory(
                &entry,
                &entry_name,
                tree_hie.clone(),
                ignore_build_vec,
                &mut children,
            )?;
        }
    }

    hash_from_save_tree(&trees_fold(), children)
}

/// Processes a directory entry and updates the `children` map
fn process_directory(
    entry: &Path,
    entry_name: &str,
    tree_hie: Option<TreeObject>,
    ignore_build_vec: &mut Vec<Gitignore>,
    children: &mut IndexMap<String, TreeNode>,
) -> io::Result<()> {
    let existing_tree = tree_node_from_tree_object(tree_hie.as_ref(), entry_name.to_string())
        .and_then(|obj| {
            if obj.node_type == TreeObjectType::TreeObject {
                deserialize_file_content::<TreeObject>(
                    &trees_fold().join(obj.hash_pointer_to_this_node.get_path()),
                )
                .ok()
            } else {
                None
            }
        });

    let new_tree_hash = dir_snapshot(entry, ignore_build_vec, existing_tree)?;

    children.insert(
        entry_name.to_string(),
        TreeNode {
            node_type: TreeObjectType::TreeObject,
            hash_pointer_to_this_node: new_tree_hash,
            hash_pointer_to_diff: None,
        },
    );

    Ok(())
}

fn process_file(
    entry: &Path,
    entry_name: &str,
    tree_hie: Option<TreeObject>,
    children: &mut IndexMap<String, TreeNode>,
) -> io::Result<()>{
    let (new_file_hash,new_file_content ) = hash_and_content_from_file_path_ref(&entry)?;

    if let Some(mut existing_version) =
        tree_node_from_tree_object(tree_hie.as_ref(), entry_name.to_string())
    {
        process_file_when_previous_version_exist(
            &new_file_hash,
            &hashed_content_from_string_lines(string_content_to_string_vec(&new_file_content)),
            &entry_name,
            &mut existing_version,
            children,
        )?;
    } else {
        save_or_create_file(
            &blob_fold().join(&new_file_hash.get_path()),
            Some(&new_file_content),
            false,
        )
            .expect("Could not save file");
        children.insert(
            entry_name.to_string(),
            TreeNode {
                node_type: TreeObjectType::BlobFile,
                hash_pointer_to_this_node: new_file_hash,
                hash_pointer_to_diff: None,
            },
        );
    }

    Ok(())
}
/// Processes a file entry and updates the `children` map
fn process_file_when_previous_version_exist(
    new_file_hash: &HashPointer,
    new_file_hashed_content: &HashedContent,
    entry_name: &str,
    existing_version: &mut TreeNode,
    children: &mut IndexMap<String, TreeNode>,
) -> io::Result<()> {

    if new_file_hash != &existing_version.hash_pointer_to_this_node {
        let previous_blob_path  = blob_fold().join(existing_version.hash_pointer_to_this_node.get_path());
        let prev_blob_hashed_content = hashed_content_from_path(&previous_blob_path);

        let mut diff = compare_hashed_content(
            &prev_blob_hashed_content,
            new_file_hashed_content
        );
        if let Some(prev_version) = existing_version.hash_pointer_to_diff.clone() {
            diff.pointer_to_previous_version = Some(prev_version);
        }

        let diff_hash = hash_from_save_content(&serialize_struct(&diff), &versions_fold())?; // diff has to be saved in version fold
        existing_version.hash_pointer_to_diff = Some(diff_hash);

        // Remove old blob
        if let Err(e) =
            fs::remove_file(previous_blob_path)
        {
            eprintln!("Warning: Failed to delete file: {}", e);
        }

        existing_version.hash_pointer_to_this_node = new_file_hash.clone();
        children.insert(entry_name.to_string(), existing_version.clone());

    }else {
        eprintln!("Warning: File hash already exists ; no need extra effort ");
    }
    Ok(())
}

pub fn tree_node_from_tree_object(
    tree_hie: Option<&TreeObject>,
    entry_name: String,
) -> Option<TreeNode> {
    if let Some(tree) = tree_hie {
        // Borrow instead of moving
        if let Some(node) = tree.children.get(&entry_name) {
            println!("entry name exists");
            return Some(node.clone()); // Clone if TreeNode needs to be owned
        }
    }

    None
}

pub fn parse_ignore_local_level(
    dir_path: &Path,
    ignore_build_vec: &mut Vec<Gitignore>,
) -> Vec<PathBuf> {
    // Read and filter directory entries
    let detected_entries =
        read_directory_entries(dir_path).expect("Could not read directory entries");
    let mut allowed_entries = Vec::new();

    // Check entries against ignore rules
    for entry in detected_entries.clone() {
        let is_dir = entry.is_dir();

        for ignore_rules in ignore_build_vec.iter() {
            match ignore_rules.matched(entry.to_str().unwrap_or(""), is_dir) {
                Match::None => {
                    allowed_entries.push(entry.clone());
                }
                Match::Ignore(_) => {
                    println!("Ignored: {}", entry.display());
                }
                Match::Whitelist(_) => {
                    allowed_entries.push(entry.clone());
                }
            }
        }
    }

    allowed_entries
}
