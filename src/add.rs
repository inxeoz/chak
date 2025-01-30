use crate::config::{get_current_dir, blob_fold, VCS_FOLDER, VCS_IGNORE_FILE};
use crate::macros::{create_file, save_to_file};
use crate::util::read_directory_entries;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use ignore::Match;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use crate::hashing::HashPointer;

pub fn start_snapshot() {
    let mut ignore_build_vec = Vec::<Gitignore>::new();
    let start_path = get_current_dir();
}
pub fn dir_snapshot(
    path: &Path,
    ignore_build_vec: &mut Vec<Gitignore>,
) -> Vec<HashPointer> {
    if !path.is_dir() {
        panic!("Path is not a directory");
    }

    let mut children_tree_object = Vec::<HashPointer>::new();
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

    // Read directory entries
    let detected_entries = read_directory_entries(path);
    let entries_set = parse_ignore_local_level(detected_entries, ignore_build_vec);

    // Process each entry
    for entry in entries_set {
        if entry.is_dir() {
            if let nested_tree  = dir_snapshot(&entry, ignore_build_vec) {
                    if ! nested_tree.is_empty() {
                        let hash_pointer = HashPointer::hash_from_pointers(nested_tree);
                        children_tree_object.push(hash_pointer);
                    }
            }
        } else {
            let entry_name = entry
                .file_name()
                .expect("Failed to get entry name")
                .to_os_string();
            if entry_name != VCS_IGNORE_FILE {
                let hash_pointer = HashPointer::save_blob(&entry, &blob_fold());
                children_tree_object.push(hash_pointer);
            }
        }
    }
   children_tree_object
}

pub fn parse_ignore_local_level(
    detected_entries: Vec<PathBuf>,
    ignore_build_vec: &mut Vec<Gitignore>,
) -> HashSet<PathBuf> {
    let mut allowed_entries = HashSet::new();
    let mut not_allowed_entries = HashSet::new();

    // Check entries against ignore rules
    for entry in detected_entries {
        let is_dir = entry.is_dir();

        for ignore_rules in ignore_build_vec.iter() {
            match ignore_rules.matched(entry.to_str().unwrap_or(""), is_dir) {
                Match::None => {
                    if !not_allowed_entries.contains(&entry) {
                        allowed_entries.insert(entry.clone());
                    }
                }
                Match::Ignore(_) => {
                    println!("Ignored: {}", entry.display());
                    if !allowed_entries.contains(&entry) {
                        not_allowed_entries.insert(entry.clone());
                    }
                }
                Match::Whitelist(_) => {
                    allowed_entries.insert(entry.clone());
                }
            }
        }
    }

    allowed_entries
}
