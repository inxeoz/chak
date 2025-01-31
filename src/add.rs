use crate::config::{blob_fold, get_current_dir, tree_fold, VCS_IGNORE_FILE};
use crate::hashing::{hash_from_pointers, hash_from_save_blob, hash_from_save_tree, HashPointer};
use crate::tree_object::{TreeObject, TreeObjectType};
use crate::util::read_directory_entries;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use ignore::Match;
use std::collections::HashSet;
use std::ops::Sub;
use std::path::{Path, PathBuf};

pub fn start_snapshot() {
    let mut ignore_build_vec = Vec::<Gitignore>::new();
    let start_path = get_current_dir();
    dir_snapshot(start_path, &mut ignore_build_vec);
}
pub fn dir_snapshot(path: &Path, ignore_build_vec: &mut Vec<Gitignore>) -> HashPointer {
    // Ensure the path is a directory
    assert!(path.is_dir(), "Path is not a directory");

    let mut children_tree_object = Vec::new();

    let mut tree_object = TreeObject {
        tree_object_type: TreeObjectType::TreeObject,
        children: Vec::new(),
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
                let nested_tree = dir_snapshot(&entry, ignore_build_vec);
                children_tree_object.push(nested_tree);
            }
        } else {
            // Skip the `.ignore` file and process other files
            let entry_name = entry
                .file_name()
                .expect("Failed to get entry name")
                .to_os_string();
            if entry_name != VCS_IGNORE_FILE {
                let hash_pointer = hash_from_save_blob(&entry, &blob_fold());
                tree_object.children.push(hash_pointer);
                //children_tree_object.push(hash_pointer);
            }
        }
    }

    hash_from_save_tree(&tree_fold(), tree_object)

    //   children_tree_object
}

pub fn parse_ignore_local_level(
    detected_entries: HashSet<PathBuf>,
    ignore_build_vec: &mut Vec<Gitignore>,
) -> HashSet<PathBuf> {
    let mut not_allowed_entries = HashSet::new();

    // Check entries against ignore rules
    for entry in detected_entries.clone() {
        let is_dir = entry.is_dir();

        for ignore_rules in ignore_build_vec.iter() {
            match ignore_rules.matched(entry.to_str().unwrap_or(""), is_dir) {
                Match::None => {}
                Match::Ignore(_) => {
                    println!("Ignored: {}", entry.display());
                    not_allowed_entries.insert(entry.clone());
                }
                Match::Whitelist(_) => {}
            }
        }
    }

    detected_entries.sub(&not_allowed_entries)
}
