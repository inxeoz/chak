use crate::custom_error::ChakError;
use crate::util::read_directory_entries;
use ignore::Match;
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Handles .ignore file processing and adds it to `ignore_build_vec`
pub fn handle_ignore_file(
    main_ignore_builder: &mut GitignoreBuilder,
    ignore_these_also: Vec<(Option<PathBuf>, &str)>,
) {
    if !ignore_these_also.is_empty() {
        // Add extra ignore rules. Handle errors gracefully.
        for (base_dir, ignore_this) in ignore_these_also {
            if let Err(err) = main_ignore_builder.add_line(base_dir, ignore_this) {
                eprintln!("Error adding ignore rule '{}': {}", ignore_this, err);
                // We could choose to continue here even with a bad rule
            }
        }
    }
}

pub fn parse_ignore(
    dir_path: &Path,
    ignore_builder: &mut GitignoreBuilder,
) -> Result<(Vec<PathBuf>, Vec<PathBuf>), ChakError> {
    // Read and filter directory entries
    let (mut detected_dir_entries, mut detected_file_entries) = read_directory_entries(dir_path)?;

    let mut allowed_dir_entries = Vec::new();
    let mut allowed_file_entries = Vec::new();
    if let Ok(build_ignore_rules) = ignore_builder.build() {
        allowed_dir_entries =
            parse_ignore_for_entries(&mut detected_dir_entries, &build_ignore_rules);
        allowed_file_entries =
            parse_ignore_for_entries(&mut detected_file_entries, &build_ignore_rules);
    }

    Ok((allowed_file_entries, allowed_dir_entries))
}

pub fn parse_ignore_combined_files_dirs(
    dir_path: &Path,
    ignore_builder: &mut GitignoreBuilder,
) -> Result<Vec<PathBuf>, ChakError> {

    let allowed_entries = parse_ignore(dir_path, ignore_builder).map(|(mut v1, v2)| {
        v1.extend(v2);
        return v1;
    })?;
    Ok(allowed_entries)
}

pub fn parse_ignore_for_entries(
    detected_entries: &mut Vec<PathBuf>,
    ignore_build: &Gitignore,
) -> Vec<PathBuf> {
    let mut allowed_entries = HashSet::new();

    for entry in detected_entries {
        match ignore_build.matched(entry.to_str().unwrap_or(""), entry.is_dir()) {
            // can i use "#" for default
            Match::None => {
                allowed_entries.insert(entry.clone());
            }
            Match::Ignore(_) => {
                if allowed_entries.contains(entry.as_path()) {
                    allowed_entries.remove(&entry.clone());
                }
                println!("Ignored: {}", entry.display());
            }
            Match::Whitelist(_) => {
                allowed_entries.insert(entry.clone());
            }
        }
    }
    allowed_entries.into_iter().collect()
}
