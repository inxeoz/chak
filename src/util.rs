
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

