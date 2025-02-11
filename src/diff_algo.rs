use crate::config::{blob_fold, get_project_dir};
use crate::custom_error::ChakError;
use crate::diff::deserialize_file_content;
use crate::hashing::{hash_from_content, HashPointer};
use crate::macros::save_or_create_file;
use indexmap::{IndexMap, IndexSet};
use itertools::{EitherOrBoth, Itertools};
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::ops::Sub;
use std::path::Path;
use clap::builder::Str;

#[derive(Serialize, PartialEq, Debug, Clone, Copy)]
pub enum DiffLineType {
    NOCHANGE,
    REPLACE,
}
#[derive(Serialize, Clone, Debug)]
pub struct Line {
    line_value: String,
    line_hash: HashPointer,
}
impl Line {
    pub fn new(line: String) -> Line {
        Line {
            line_value: line.clone(),
            line_hash: hash_from_content(&line),
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct DiffLine {
    #[serde(skip)]
    pub diff_line_type: DiffLineType,
    #[serde(skip)]
    pub copy_from_live: Line,
}

impl DiffLine {
    pub fn from(prev_line: Line, live_line: Line) -> Self {
        let diff_line_type = if prev_line.line_hash == live_line.line_hash {
            DiffLineType::NOCHANGE
        } else {
            DiffLineType::REPLACE
        };

        Self {
            diff_line_type,
            copy_from_live: if diff_line_type == DiffLineType::REPLACE {
                prev_line
            } else {
                Line::new(String::new())
            },
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct Block {
    pub block_content: Vec<DiffLine>,
    pub block_type: DiffLineType,
    // #[serde(skip)]
    pub block_hash: HashPointer,
}

impl Block {
    pub fn from(diff_line: DiffLine) -> Self {
        Self {
            block_content: vec![diff_line.clone()],
            block_type: diff_line.diff_line_type,
            block_hash: diff_line.copy_from_live.line_hash,
        }
    }

    pub fn add(&mut self, diff_line: DiffLine) -> Result<(), String> {
        if diff_line.diff_line_type == self.block_type {
            self.block_hash
                .update_hash(diff_line.copy_from_live.line_hash.get_one_hash());
            self.block_content.push(diff_line);
            Ok(())
        } else {
            Err("DiffLine type mismatch".into())
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct ContentBlock {
    pub content: Vec<Block>,
    pub content_hash: HashPointer,
}

impl ContentBlock {
    pub fn new() -> Self {
        Self {
            content_hash: hash_from_content(&String::new()),
            content: vec![],
        }
    }
    pub fn from(block: Block) -> Self {
        Self {
            content_hash: block.block_hash.clone(),
            content: vec![block],
        }
    }

    pub fn add(&mut self, diff_line: DiffLine) {
        if let Some(last_block) = self.content.last_mut() {
            if last_block.block_type == diff_line.diff_line_type {
                last_block.add(diff_line).unwrap();
            } else {
                self.content.push(Block::from(diff_line));
            }
        } else {
            let block = Block::from(diff_line);
            self.content_hash = block.block_hash.clone();
            self.content.push(block);
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct HashedContent {
    pub pointer_to_previous_version: Option<HashPointer>,
    pub hash_lines: IndexSet<String>,
    pub hash_to_content: HashMap<String, String>,
}
pub fn to_hashed_content(file: &File) -> HashedContent {
    let mut hash_lines = IndexSet::new();
    let mut hash_to_content = HashMap::<String, String>::new();
    for res_line in BufReader::new(file).lines() {
        if let Ok(line) = res_line {
            let hash_string = hash_from_content(&line).get_one_hash();
            hash_lines.insert(hash_string.clone());

            // Map hash string to actual line content (only if not already mapped)
            hash_to_content.entry(hash_string).or_insert(line);
        }
    }
    HashedContent {
        pointer_to_previous_version: None,
        hash_lines,
        hash_to_content,
    }
}

//biased toward previous content
pub fn compare_hashed_content(
    pre_content: HashedContent,
    new_content: HashedContent,
) -> HashedContent {
    //content has to be unquie in prev content to add hash -> content map for diff because diff going to biased toward prev content
    let prev_hash_lines = pre_content.hash_lines;
    let prev_line_contents = pre_content.hash_to_content;
    let new_hash_lines = new_content.hash_lines;
    let unique_hash_lines_in_prev_lines = prev_hash_lines.sub(&new_hash_lines);

    let mut unique_line_contents = HashMap::<String, String>::new();

    for unique_prev_line_hash in unique_hash_lines_in_prev_lines {
       if let Some(unique_line_content) = prev_line_contents.get(&unique_prev_line_hash) {
           unique_line_contents.insert(unique_prev_line_hash.clone(), unique_line_content.clone());
       }
    }


    HashedContent {
        pointer_to_previous_version: None,
        hash_lines: prev_hash_lines,
        hash_to_content: unique_line_contents,
    }
}

pub fn file_to_lines(file: &File) -> Vec<String> {
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| line.unwrap_or_default())
        .collect()
}

//TODO use this function for making previous version file
pub fn previous_content_from_new_content_using_diff(
    diff: HashedContent,
    fixed_next_content: HashedContent,
) -> Vec<String> {
    let mut previous_lines = Vec::<String>::new();
    for line_hash in &diff.hash_lines {
        let content = diff
            .hash_to_content
            .get(line_hash)
            .or_else(|| fixed_next_content.hash_to_content.get(line_hash))
            .cloned()
            .unwrap_or_default();

        previous_lines.push(content);
    }

    previous_lines
}

pub fn restore_previous_version(blob_path: &Path, diff_path: &Path) -> Result<(), io::Error> {
    let blob_file = File::open(blob_path)?;
    let fixed_next_content = to_hashed_content(&blob_file);
    let diff = deserialize_file_content(&diff_path)?;
    let previous_version = previous_content_from_new_content_using_diff(diff, fixed_next_content);
    let mut restore_file = save_or_create_file(
        &get_project_dir().join("restore").join("file.txt"),
        None,
        true,
    )?;

    for lines in previous_version.iter() {
        restore_file.write_all(lines.as_bytes())? // /n would work not here  ?
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::diff_algo::{compare_hashed_content, to_hashed_content};
    use std::fs::File;
    use std::{env, io};

    #[test]
    fn test_diff_algo() -> io::Result<()> {
        let prev_file = File::open(env::current_dir()?.join("file1.txt"))?;
        let new_file = File::open(env::current_dir()?.join("file2.txt"))?;

        // Generate mappings
        let prev_file_content = to_hashed_content(&prev_file);
        let new_file_content = to_hashed_content(&new_file);

        println!(
            "prev\n{}",
            serde_json::to_string_pretty(&prev_file_content)?
        );

        println!("new\n{}", serde_json::to_string_pretty(&new_file_content)?);

        let diff_biased_prev = compare_hashed_content(prev_file_content, new_file_content);
        println!("diff\n{}", serde_json::to_string_pretty(&diff_biased_prev)?);

        Ok(())
    }
}
