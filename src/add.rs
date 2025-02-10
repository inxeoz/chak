use crate::commit::{attach_latest_root_pointer_to_stage, Commit};
use crate::config::{
    blob_fold, commits_fold, get_project_dir, history_fold, staging_area_fold, VCS_IGNORE_FILE,
};
use crate::custom_error::ChakError;
use crate::diff::{deserialize_file_content, get_diff, serialize_struct};
use crate::hashing::{get_latest_pointer_from_file, hash_and_content_from_file_path_ref, hash_from_file, hash_from_save_content, hash_from_save_tree, HashPointer};
use crate::tree_object::{TreeNode, TreeObject, TreeObjectType};
use crate::util::read_directory_entries;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use ignore::Match;
use indexmap::{map, IndexMap};
use itertools::Itertools;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::{fs, io};
use std::fs::File;
use clap::Error;
use crate::macros::{file_to_string, save_or_create_file};

pub fn start_snapshot() -> io::Result<()> {
    let mut ignore_build_vec = Vec::<Gitignore>::new();
    let start_path = get_project_dir();

    //implement the tree pointer with traversing fold/file and checking hash from tree pointer and so on .. TODO
    //get latest tree pointer from history_log
    let mut tree_object: Option<TreeObject> = None;
    let commit_file = File::open(&history_fold().join("commit_log")).expect("Unable to open commit_log file");

    if let Some(commit_pointer) =
        get_latest_pointer_from_file(&commit_file, true)
    {
        if let Ok(latest_commit) =
            deserialize_file_content::<Commit>(&commits_fold().join(commit_pointer.get_path()))
        {
            let root_tree_pointer = latest_commit.root_tree_pointer.get_path();

            match deserialize_file_content::<TreeObject>(&blob_fold().join(root_tree_pointer)) {
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

        if entry.is_dir() {
            process_directory( &entry, &entry_name, &tree_hie, ignore_build_vec, &mut children)?;
        } else if entry_name != VCS_IGNORE_FILE {
            process_file(&entry, &entry_name, &tree_hie, &mut children)?;
        }
    }

    hash_from_save_tree(&blob_fold(), children)
}

/// Handles .ignore file processing and adds it to `ignore_build_vec`
fn handle_ignore_file(path: &Path, ignore_build_vec: &mut Vec<Gitignore>) {
    let ignore_file = path.join(VCS_IGNORE_FILE);
    if ignore_file.exists() {
        let mut local_builder = GitignoreBuilder::new(path);
        local_builder.add(&ignore_file);
        if let Ok(ignore) = local_builder.build()
        {
            ignore_build_vec.push(ignore);
        } else {
            eprintln!("Could not create Gitignore from path: {}", ignore_file.display());
        }
    }
}


/// Processes a directory entry and updates the `children` map
fn process_directory(
    entry: &Path,
    entry_name: &str,
    tree_hie: &Option<TreeObject>,
    ignore_build_vec: &mut Vec<Gitignore>,
    children: &mut IndexMap<String, TreeNode>,
) -> io::Result<()> {
    let existing_tree = tree_node_from_tree_object(tree_hie.as_ref(), entry_name.to_string())
        .and_then(|obj| {
            if obj.blob_type == TreeObjectType::TreeObject {
                deserialize_file_content::<TreeObject>(&blob_fold().join(obj.pointer_to_blob.get_path())).ok()
            } else {
                None
            }
        });

    let new_tree_hash = dir_snapshot(entry, ignore_build_vec, existing_tree)?;

    children.insert(entry_name.to_string(), TreeNode {
        blob_type: TreeObjectType::TreeObject,
        pointer_to_blob: new_tree_hash,
        pointer_to_diff: None,
    });

    Ok(())
}

/// Processes a file entry and updates the `children` map
fn process_file(
    entry: &Path,
    entry_name: &str,
    tree_hie: &Option<TreeObject>,
    children: &mut IndexMap<String, TreeNode>,
) -> io::Result<()> {
    let (new_hash, content) = hash_and_content_from_file_path_ref(entry)?;

    if let Some(mut existing_node) =  tree_node_from_tree_object(tree_hie.as_ref(), entry_name.to_string()) {
         if new_hash != existing_node.pointer_to_blob  {
            // Handle changes in the file

            let prev_file = File::open(&blob_fold().join(new_hash.get_path()))?;
            let new_file = File::open(&blob_fold().join(existing_node.pointer_to_blob.get_path()))?;

            let mut diff = get_diff(
              &prev_file,
              &new_file,
            );
            if let Some(prev_version) = existing_node.pointer_to_diff {
                diff.pointer_to_previous_version = Some(prev_version);
            }

            let diff_hash = hash_from_save_content( &serialize_struct(&diff), &blob_fold())?;
            existing_node.pointer_to_diff = Some(diff_hash);

            // Remove old blob
            if let Err(e) = fs::remove_file(blob_fold().join(existing_node.pointer_to_blob.get_path())) {
                eprintln!("Warning: Failed to delete file: {}", e);
            }

            existing_node.pointer_to_blob = new_hash.clone();
            children.insert(entry_name.to_string(), existing_node);

             Ok(())
        }else { println!("hash is same");  Ok(())}

    }else {
        save_or_create_file(&blob_fold().join(&new_hash.get_path()),  Some(&content), false).expect("Could not save file");
        children.insert(entry_name.to_string(), TreeNode {
            blob_type: TreeObjectType::BlobFile,
            pointer_to_blob: new_hash,
            pointer_to_diff: None,
        });
        Ok(())
    }

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
    let detected_entries = read_directory_entries(dir_path).expect("Could not read directory entries");
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
