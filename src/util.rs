use crate::config::{get_project_dir};
use std::collections::HashSet;
use std::fs::read_dir;
use std::io;
use std::path::{Path, PathBuf};

pub fn check_vcs_presence(fold: &Path) -> bool {
        if fold.join(".chak").exists() {
            return true;
        }
        // Read the directory and check subdirectories recursively
        if let Ok(entries) = read_dir(fold) {
            for entry in entries {
                if let Ok(entry) = entry {
                    // Recursively check each subdirectory
                    return check_vcs_presence(&entry.path())
                }
            }
        }
    false
}

pub fn read_directory_entries(path: &Path) -> io::Result<Vec<PathBuf>> {

    let entries = read_dir(path)?;
    let mut detected_entries = Vec::new();

    for entry in entries {
        let entry = entry?.path();
        detected_entries.push(entry.clone());
    }

    Ok(detected_entries)
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
//             let path_blob_fold = &get_project_dir().join("store").join("blobs").join(&hash[..2]).to_str().expect("Could not convert hash to str").to_string();
//             let blob_file_name = &hash[2..];
//             createfile!(&path_blob_fold, blob_file_name);
//
//         }
//     }
//
