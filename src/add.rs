use crate::commit::{attach_latest_root_pointer_to_stage, Commit};
use crate::config::{
    blob_fold, commit_history_fold, commits_fold, get_current_dir, staging_area_fold,
    VCS_IGNORE_FILE,
};
use crate::diff::deserialize_file_content;
use crate::hashing::{
    get_latest_pointer_from_file, hash_from_pointers, hash_from_save_blob, hash_from_save_tree,
    HashPointer,
};
use crate::tree_object::{TreeNode, TreeObject, TreeObjectType};
use crate::util::read_directory_entries;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use ignore::Match;
use indexmap::IndexMap;
use itertools::Itertools;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Sub;
use std::path::{Path, PathBuf};

pub fn start_snapshot() {
    let mut ignore_build_vec = Vec::<Gitignore>::new();
    let start_path = get_current_dir();

    //implement the tree pointer with traversing fold/file and checking hash from tree pointer and so on .. TODO
    //get latest tree pointer from history_log
    let tree_hie = {
        if let Ok(commit_pointer) =
            get_latest_pointer_from_file(&commit_history_fold().join("commit_log"), true)
        {
            let latest_commit: Commit =
                deserialize_file_content(&commits_fold().join(commit_pointer.get_path()))
                    .expect("Failed to deserialize latest commit file.");
            deserialize_file_content(&blob_fold().join(latest_commit.root_tree_pointer.get_path())).unwrap_or_else(|e| { None })

        } else {
            None
        }
    };

let root_tree_pointer = dir_snapshot(start_path, &mut ignore_build_vec, tree_hie);
    attach_latest_root_pointer_to_stage(root_tree_pointer);
}
pub fn dir_snapshot(path: &Path, ignore_build_vec: &mut Vec<Gitignore>, tree_hie: Option<TreeObject>) -> HashPointer {
    // Ensure the path is a directory
    assert!(path.is_dir(), "Path is not a directory");

    let mut children_tree_object = Vec::new();

    let mut tree_object = TreeObject {
        children: IndexMap::new(),
    };
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
    let entries_set = parse_ignore_local_level(read_directory_entries(path), ignore_build_vec);

    // Process each entry
    for entry in entries_set {
        if entry.is_dir() {
            // Skip `.chak` directories at the current level
            if !(get_current_dir() == path && entry.ends_with(".chak")) {
                let nested_tree = dir_snapshot(&entry, ignore_build_vec, None);
                children_tree_object.push(nested_tree);
            }
        } else {
            // Skip the `.ignore` file and process other files
            let entry_name = entry
                .file_name()
                .expect("Could not get file name")
                .to_str()
                .expect("Could not convert to str")
                .to_string();
            if entry_name != VCS_IGNORE_FILE {
                let hash_pointer = hash_from_save_blob(&entry, &blob_fold());
                if let Some(ref tree) = tree_hie {
                    if tree.children.contains_key(&entry_name) {
                        if let Some(child_object )= tree.children.get(&entry_name){
                            if hash_pointer != child_object.pointer_to_blob {
                                //get diff for pointer_to_blob and with entry and save the diff as
                            }
                        }
                    }
                }
                tree_object.children.insert(
                    entry_name,
                    TreeNode {
                        is_file: false,
                        blob_type: TreeObjectType::BlobFile,
                        pointer_to_blob: hash_pointer.clone(),
                        pointer_to_previous_node: hash_pointer
                    },
                );
            }
        }
    }

    hash_from_save_tree(&blob_fold(), &mut tree_object)
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
