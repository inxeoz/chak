use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use indexmap::{IndexMap, IndexSet};
use crate::chak_traits::ObjectCommonTraits;
use crate::config::get_blob_fold_path;
use itertools::{ Itertools};
use std::fs::File;
use std::hash::Hash;
use std::ops::Sub;
use std::path::Path;
use crate::chak_traits::HashPointerTraits;
use crate::hash_pointer::{HashPointer};
use crate::util::file_to_lines;

pub struct CompareOrderStructure {
    pub previous_content: BlobObject,
    pub new_content: BlobObject,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BlobObject {
    pub hash_lines: IndexSet<String>,
    pub hash_to_content: IndexMap<String, String>,
}


impl ObjectCommonTraits for BlobObject {
    fn containing_folder() -> PathBuf {
        get_blob_fold_path()
    }
}


impl BlobObject {

    fn new(hash_lines: &IndexSet<String>, hash_to_content: &mut IndexMap<String, String>) -> BlobObject {
        hash_to_content.sort_keys();
        BlobObject {
            hash_lines: hash_lines.to_owned(),
            hash_to_content: hash_to_content.to_owned(),
        }

    }
    //need to test this method
    pub fn to_string_content(&self) -> String {
        let mut string_lines = Vec::<String>::new();
        for Hash_line in &self.hash_lines {
            string_lines.push(self.hash_to_content.get(Hash_line).unwrap_or(&"".to_string()).to_string());
        }
        string_lines.join("\n")
    }
    //biased toward previous content
    //it is important as it tell the ordering of paramter like which hashedContent is previos content or new content
    pub fn compare_hashed_content_biased_previous(
        compare_order_structure: &CompareOrderStructure,
    ) -> BlobObject {
        let CompareOrderStructure {
            previous_content,
            new_content,
        } = compare_order_structure;

        //content has to be unquie in prev content to add hash -> content map for diff because diff going to biased toward prev content
        let prev_hash_lines = &previous_content.hash_lines;
        let prev_line_contents = &previous_content.hash_to_content;
        let new_hash_lines = &new_content.hash_lines;
        let unique_hash_lines_in_prev_lines = prev_hash_lines.sub(&new_hash_lines);

        let mut unique_line_contents = IndexMap::<String, String>::new();

        for unique_prev_line_hash in unique_hash_lines_in_prev_lines {
            if let Some(unique_line_content) = prev_line_contents.get(&unique_prev_line_hash) {
                unique_line_contents
                    .insert(unique_prev_line_hash.clone(), unique_line_content.clone());
            }
        }

        BlobObject::new(&prev_hash_lines, &mut unique_line_contents)
    }

    pub fn from_string_lines(lines: Vec<String>) -> BlobObject {
        let mut hash_lines = IndexSet::<String>::new();
        let mut hash_to_content = IndexMap::<String, String>::new();
        for line in lines {
            let hash_line = HashPointer::from_string(&line).get_one_hash();
            hash_lines.insert(hash_line.clone());
            hash_to_content.insert(hash_line, line);
        }

        BlobObject::new(&hash_lines, &mut hash_to_content)
    }

    pub fn from_file(file: &File) -> BlobObject {
        BlobObject::from_string_lines(file_to_lines(file))
    }
    pub fn get_diff(prev_file: &File, new_file: &File) -> BlobObject {
        let first = Self::from_file(&prev_file);
        let second = Self::from_file(&new_file);
        let diff = Self::compare_hashed_content_biased_previous(&CompareOrderStructure {
            previous_content: first,
            new_content: second,
        });
        diff
    }

    pub fn from_file_path(path: &Path) -> BlobObject {
        let file = File::open(&path).expect("Failed to open file");
        Self::from_file(&file)
    }
}


