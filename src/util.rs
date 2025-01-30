use crate::config::{get_current_dir, VCS_IGNORE_FILE};
use std::collections::HashSet;
use std::fs;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use ignore::Match;

pub fn check_vcs_presence_in_subdir(vcs_name: &str) -> Vec<PathBuf> {
    let mut presence_vec = Vec::new();
    let current_dir = get_current_dir();

    // Dereference CURRENT_PATH to satisfy AsRef<Path> requirement
    for entry in read_dir(&current_dir)
        .map_err(|_| format!("Failed to read directory: {}", &current_dir.display()))
        .expect("Could not read directory")
    {
        if let Ok(entry) = entry {
            if entry.path().join(vcs_name).exists() {
                presence_vec.push(entry.path());
            }
        }
    }

    presence_vec
}

pub fn check_vcs_presence() -> bool {
    if get_current_dir().join(".chak").exists() {
        true
    } else {
        println!("VCS presence could not be found in current directory");
        let present_subdirs = check_vcs_presence_in_subdir(".chak");
        if !present_subdirs.is_empty() {
            println!("VCS presence detected in these subdirectories:");
            for _presence in present_subdirs {
                println!("in this folder {} .chak/ folder exist", _presence.display());
            }
        }
        false
    }
}

pub fn read_directory_entries(path: &Path) -> Vec<PathBuf> {

    let entries = fs::read_dir(path).expect("Could not read directory");
    let mut detected_entries = Vec::new();

    for entry in entries {
        let entry = entry.expect("Could not read directory entry").path();
        detected_entries.push(entry.clone());
    }

    detected_entries
}

// pub fn parse_ignore(path: &Path, ignore_build_vec: &mut Vec<Gitignore>) {
//     if !path.is_dir() {
//         return;
//     }
//
//     // Add .ignore file to the ignore builder if it exists
//     let ignore_file = path.join(VCS_IGNORE_FILE);
//     if ignore_file.exists() {
//         let mut local_builder = GitignoreBuilder::new(path);
//         local_builder.add(&ignore_file);
//         if let Ok(ignore) = local_builder.build() {
//             ignore_build_vec.push(ignore);
//         } else {
//             eprintln!("Could not create Gitignore from path: {}", ignore_file.display());
//         }
//     }
//
//     // Read directory entries
//     let detected_entries = read_directory_entries(path, None);
//     let entries_set = crate::status::parse_ignore_local_level(detected_entries, ignore_build_vec);
//
//     // Process each entry
//     for entry in entries_set {
//         // println!("allowed {}", entry.display());
//         if entry.is_dir() {
//             crate::status::parse_ignore(&entry, ignore_build_vec);
//         }else {
//             let hash = get_file_hash(&entry).expect("Could not get file hash");
//             let content = fs::read_to_string(&entry).expect("Could not read content");
//             let path_blob_fold = &get_current_dir().join("store").join("blobs").join(&hash[..2]).to_str().expect("Could not convert hash to str").to_string();
//             let blob_file_name = &hash[2..];
//             createfile!(&path_blob_fold, blob_file_name);
//
//         }
//     }
//
