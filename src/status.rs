use std::collections::HashSet;
use std::path::{Path, PathBuf};
use crate::config::{blob_fold, get_current_dir, VCS_FOLDER, VCS_IGNORE_FILE};
use crate::hashing::{ HashPointer};
use crate::util::read_directory_entries;

pub fn get_status(path: &Path) -> Vec<PathBuf> {
    let mut status = Vec::new();
    if path.is_dir() {
        for entry in path.read_dir().expect("Could not read directory") {
            if let Ok(entry) = entry {
                status.extend(get_status(&entry.path()));
            }
        }
    } else if let file_blob_pointer = HashPointer::from_file(path) {
        let blob_path = blob_fold().join(file_blob_pointer.get_path());
        if !blob_path.exists() {
            if let Ok(relative_path) = path.strip_prefix(get_current_dir()) {
                println!("changed {}", &relative_path.display());
                status.push(relative_path.to_path_buf());

            } else {
                println!("changed {}", path.display());
                status.push(path.to_path_buf());

            }
        }
    }

    status
}



