use crate::commit::{attach_latest_root_pointer_to_stage, Commit};
use crate::config::{
    blob_fold, commits_fold, get_project_dir, history_fold, staging_area_fold, VCS_IGNORE_FILE,
};
use crate::custom_error::ChakError;
use crate::diff::{deserialize_file_content, get_diff, serialize_struct};
use crate::hashing::{
    get_latest_pointer_from_file, hash_from_pointers, hash_from_save_blob, hash_from_save_content,
    hash_from_save_tree, HashPointer,
};
use crate::tree_object::{TreeNode, TreeObject, TreeObjectType};
use crate::util::read_directory_entries;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use ignore::Match;
use indexmap::{map, IndexMap};
use itertools::Itertools;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn start_snapshot() -> io::Result<()> {
    let mut ignore_build_vec = Vec::<Gitignore>::new();
    let start_path = get_project_dir();

    //implement the tree pointer with traversing fold/file and checking hash from tree pointer and so on .. TODO
    //get latest tree pointer from history_log
    let mut tree_object: Option<TreeObject> = None;
    if let Ok(commit_pointer) =
        get_latest_pointer_from_file(&history_fold().join("commit_log"), true)
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
    // Ensure the path is a directory
    assert!(path.is_dir(), "Path is not a directory");

    let mut children = IndexMap::<String, TreeNode>::new();

    // Add .ignore file to the ignore builder if it exists
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
    // Read and filter directory entries based on ignore rules

    let entries_set = read_directory_entries(path)?;
    let allowed_entries_set = parse_ignore_local_level(entries_set.clone(), ignore_build_vec);

    // Process each entry
    for entry in allowed_entries_set {
        // Skip the `.ignore` file and process other files
        let entry_name = entry
            .file_name()
            .expect("Could not get file name")
            .to_str()
            .expect("Could not convert to str")
            .to_string();

        if entry.is_dir() {
            let mut entry_tree_node: Option<TreeObject> = None;
            // Skip `.chak` directories at the current level
            if !(path == get_project_dir() && entry.ends_with(".chak")) {
                if let Some(child_object) =
                    tree_node_from_tree_object(tree_hie.as_ref(), entry_name.clone())
                {
                    if child_object.blob_type == TreeObjectType::TreeObject {
                        entry_tree_node = deserialize_file_content::<TreeObject>(
                            &blob_fold().join(child_object.pointer_to_blob.get_path()),
                        )
                        .ok();
                    }
                }

                let new_tree_hash_pointer = dir_snapshot(&entry, ignore_build_vec, entry_tree_node)?;
                children.insert(
                    entry_name,
                    TreeNode {
                        blob_type: TreeObjectType::TreeObject,
                        pointer_to_blob: new_tree_hash_pointer.clone(),
                        pointer_to_diff: None,
                    },
                );
            }
        } else {
            if entry_name != VCS_IGNORE_FILE {
                let new_hash_pointer = hash_from_save_blob(&entry, &blob_fold())?;

                if let Some(mut child_object) =
                    tree_node_from_tree_object(tree_hie.as_ref(), entry_name.clone())
                {
                    if new_hash_pointer != child_object.pointer_to_blob {
                        let mut diff = get_diff(
                            &blob_fold().join(new_hash_pointer.get_path()),
                            &blob_fold().join(child_object.pointer_to_blob.get_path()),
                        )?;

                        if let Some(previous_version) = child_object.pointer_to_diff {
                            diff.pointer_to_previous_version = Some(previous_version);
                        }
                        let new_pointer_to_diff =
                            hash_from_save_content(&blob_fold(), serialize_struct(&diff))?;
                        child_object.pointer_to_diff = Some(new_pointer_to_diff.clone());
                        if let Err(e) = fs::remove_file(
                            blob_fold().join(child_object.pointer_to_blob.get_path()),
                        ) {
                            eprintln!("Warning: Failed to delete file: {}", e);
                        }
                        child_object.pointer_to_blob = new_hash_pointer.clone();

                        children.insert(entry_name, child_object);
                    }
                } else {
                    children.insert(
                        entry_name,
                        TreeNode {
                            blob_type: TreeObjectType::BlobFile,
                            pointer_to_blob: new_hash_pointer.clone(),
                            pointer_to_diff: None,
                        },
                    );
                }
            }
        }
    }

    hash_from_save_tree(&blob_fold(), children)
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
    detected_entries: Vec<PathBuf>,
    ignore_build_vec: &mut Vec<Gitignore>,
) -> Vec<PathBuf> {
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
