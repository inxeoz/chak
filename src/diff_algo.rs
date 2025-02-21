
use crate::handle_version::VersionHashPointer;
use crate::hash_pointer_algo::{hash_from_content};
use indexmap::{ IndexSet};
use itertools::{ Itertools};
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader, Write};
use std::ops::Sub;
use std::path::Path;
use crate::hash_pointer::HashPointerTraits;


pub struct CompareOrderStructure {
    pub previous_content: HashedContent,
    pub new_content: HashedContent,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct HashedContent {
    pub hash_lines: IndexSet<String>,
    pub hash_to_content: HashMap<String, String>,
}
impl HashedContent {
    //biased toward previous content
    //it is important as it tell the ordering of paramter like which hashedContent is previos content or new content
    pub fn compare_hashed_content_biased_previous(
        compare_order_structure: &CompareOrderStructure,
    ) -> HashedContent {
        let CompareOrderStructure {
            previous_content,
            new_content,
        } = compare_order_structure;

        //content has to be unquie in prev content to add hash -> content map for diff because diff going to biased toward prev content
        let prev_hash_lines = &previous_content.hash_lines;
        let prev_line_contents = &previous_content.hash_to_content;
        let new_hash_lines = &new_content.hash_lines;
        let unique_hash_lines_in_prev_lines = prev_hash_lines.sub(&new_hash_lines);

        let mut unique_line_contents = HashMap::<String, String>::new();

        for unique_prev_line_hash in unique_hash_lines_in_prev_lines {
            if let Some(unique_line_content) = prev_line_contents.get(&unique_prev_line_hash) {
                unique_line_contents
                    .insert(unique_prev_line_hash.clone(), unique_line_content.clone());
            }
        }

        HashedContent {
            hash_lines: prev_hash_lines.clone(),
            hash_to_content: unique_line_contents,
        }
    }

    pub fn hashed_content_from_string_lines(lines: Vec<String>) -> HashedContent {
        let mut hash_lines = IndexSet::<String>::new();
        let mut hash_to_content = HashMap::<String, String>::new();
        for line in lines {
            let hash_line = hash_from_content(&line).get_one_hash();
            hash_lines.insert(hash_line.clone());
            hash_to_content.insert(hash_line, line);
        }
        HashedContent {
            hash_lines,
            hash_to_content,
        }
    }

    pub fn hashed_content_from_file(file: &File) -> HashedContent {
        let mut hash_lines = IndexSet::new();
        let mut hash_to_content = HashMap::<String, String>::new();

        for line in file_to_lines(file) {
            let hash_string = hash_from_content(&line).get_one_hash();
            hash_lines.insert(hash_string.clone());
            // Map hash string to actual line content (only if not already mapped)
            hash_to_content.entry(hash_string).or_insert(line);
        }
        HashedContent {
            hash_lines,
            hash_to_content,
        }
    }
    pub fn get_diff(prev_file: &File, new_file: &File) -> HashedContent {
        let first = Self::hashed_content_from_file(&prev_file);
        let second = Self::hashed_content_from_file(&new_file);
        let diff = Self::compare_hashed_content_biased_previous(&CompareOrderStructure {
            previous_content: first,
            new_content: second,
        });
        diff
    }

    pub fn hashed_content_from_path(path: &Path) -> HashedContent {
        let file = File::open(&path).expect("Failed to open file");
        Self::hashed_content_from_file(&file)
    }
}

pub fn file_to_lines(file: &File) -> Vec<String> {
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.unwrap_or_default())
        .collect()
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct HashedContentForVersion {
    pub pointer_to_previous_version: Option<VersionHashPointer>,
    pub hashed_content: HashedContent,
}
impl HashedContentForVersion {
    pub fn new(
        diff_content: HashedContent,
        pointer_to_previous_version: Option<VersionHashPointer>,
    ) -> Self {
        Self {
            pointer_to_previous_version,
            hashed_content: diff_content,
        }
    }
}

//
// #[cfg(test)]
// mod tests {
//     use crate::config::get_project_dir;
//     use crate::util::{deserialize_file_content, serialize_struct};
//     use crate::diff_algo::{compare_hashed_content_biased_previous, hashed_content_from_file, HashedContent};
//     use crate::hashing::{HashPointer, _hash_pointer_from_hash_string};
//     use crate::util::save_or_create_file;
//     use crate::restore::restore_previous_version;
//     use std::fs::File;
//     use std::{env, io};
//
//     #[test]
//     fn test_diff_algo() -> io::Result<()> {
//         let file1 = File::open(env::current_dir()?.join("file1.txt"))?;
//         let file2 = File::open(env::current_dir()?.join("file2.txt"))?;
//         let file3 = File::open(env::current_dir()?.join("file3.txt"))?;
//         // Generate mappings
//         let file1_content = hashed_content_from_file(&file1);
//         let file2_content = hashed_content_from_file(&file2);
//         let file3_content = hashed_content_from_file(&file3);
//
//         let diff_base_1 = compare_hashed_content_biased_previous(&file1_content, &file2_content);
//         let serialized_1 = serialize_struct(&diff_base_1);
//         save_or_create_file(
//             &get_project_dir().join("restore").join("diff1.json"),
//             Some(&serialized_1),
//             false,
//             None
//         )?;
//
//         let mut diff_base_2 = compare_hashed_content(&file2_content, &file3_content);
//         // diff_base_2.pointer_to_previous_version =
//         //     Some(_hash_pointer_from_hash_string("restore".to_string()));
//         let serialized_2 = serialize_struct(&diff_base_2);
//         save_or_create_file(
//             &get_project_dir().join("restore").join("diff2.json"),
//             Some(&serialized_2),
//             false,
//             None
//         )?;
//
//         Ok(())
//     }
//
//      #[test]
//     fn restore_previous_version_test()  {
//         // let file3 = File::open(env::current_dir()?.join("file3.txt"))?;
//         // let file3_content = hashed_content_from_file(&file3);
//         //
//         // // Generate mappings
//         // let diff2 = deserialize_file_content::<HashedContent>(
//         //     &get_project_dir().join("restore").join("diff2.json"),
//         // )
//         // .ok()
//         // .expect("restore failed");
//         //
//         // let diff1 = deserialize_file_content::<HashedContent>(
//         //     &get_project_dir().join("restore").join("diff1.json"),
//         // )
//         // .ok()
//         // .expect("restore failed");
//         //
//         // if let Ok(file2_content_vec) = restore_previous_version(&file3_content, &diff2) {
//         //     let file2_content = hashed_content_from_string_lines(file2_content_vec.clone());
//         //     // println!("diff content\n{}", serde_json::to_string_pretty(&)?);
//         //
//         //     let serialzed = serialize_struct(&file2_content);
//         //     save_or_create_file(
//         //         &get_project_dir().join("restore").join("restored2.json"),
//         //         Some(&serialzed),
//         //         false,
//         //         None
//         //     )?;
//         //
//         //     if let Ok(file1_content_vec) = restore_previous_version(&file2_content, &diff1) {
//         //         let file1_content = hashed_content_from_string_lines(file1_content_vec.clone());
//         //         // println!("diff content\n{}", serde_json::to_string_pretty(&)?);
//         //
//         //         let serialzed = serialize_struct(&file1_content);
//         //         save_or_create_file(
//         //             &get_project_dir().join("restore").join("restored1.json"),
//         //             Some(&serialzed),
//         //             false,
//         //             None
//         //         )?;
//         //     }
//         // }
//         //
//         // Ok(())
//     }
// }
