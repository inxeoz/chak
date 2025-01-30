use crate::hashing::HashPointer;
use itertools::{EitherOrBoth, Itertools};
use serde::Serialize;
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io;
use std::io::{BufRead, BufReader};
use std::option::Option;
use std::path::Path;
use std::rc::Rc;
use indexmap::IndexMap;

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
            line_hash: HashPointer::from_content(line),
        }
    }
}

#[derive(Serialize, PartialEq, Debug, Clone)]
pub struct HashedContent {
    pub line_to_hash: IndexMap<i64, String>,
    pub hash_to_content: HashMap<String, String>,
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
            content_hash: HashPointer::from_content(String::new()),
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
pub fn to_interconnected_line(file_path: &Path) -> HashedContent {
    let file = File::open(file_path).expect("cannot open file");

    let mut line_to_hash = IndexMap::<i64, String>::new();
    let mut hash_to_content = HashMap::<String, String>::new();

    for (index, res_line) in BufReader::new(file).lines().enumerate() {
        if let Ok(line) = res_line {
            let hash_string = HashPointer::from_content(line.clone()).get_one_hash();

            // Map line number to hash string
            line_to_hash.insert(index as i64, hash_string.clone());

            // Map hash string to actual line content (only if not already mapped)
            hash_to_content.entry(hash_string).or_insert(line);
        }
    }

    HashedContent {
        line_to_hash,
        hash_to_content,
    }
}

pub fn compare_hashed_content(
    pre_content: HashedContent,
    new_content: HashedContent,
) -> HashedContent{

    let prev_line_to_hash = pre_content.line_to_hash;
    let pre_hash_to_content = pre_content.hash_to_content;
    let new_line_to_hash = new_content.line_to_hash;

    let mut diff_line_to_hash = IndexMap::<i64, String>::new();
    let mut diff_hash_to_content = HashMap::<String, String>::new();

    let insert_hash_to_content_in_diff = |p_hash: &String, mut diff_hash_to_content: &mut HashMap<String, String>| {
        if !diff_hash_to_content.contains_key(p_hash.as_str()) {
            diff_hash_to_content.insert(
                p_hash.clone(),
                pre_hash_to_content.get(p_hash.as_str()).unwrap().clone(),
            );
        }
    };

    for pair in prev_line_to_hash
        .iter()
        .zip_longest(new_line_to_hash.iter())
    {
        match pair {
            EitherOrBoth::Both((p_index, p_hash), (n_index, n_hash)) => {
                if p_hash == n_hash {
                    diff_line_to_hash.insert(*n_index, n_hash.clone());
                } else {
                    diff_line_to_hash.insert(*p_index, p_hash.clone());
                    insert_hash_to_content_in_diff(p_hash, &mut diff_hash_to_content);
                }
            }

            EitherOrBoth::Left((p_index, p_hash)) => {
                diff_line_to_hash.insert(*p_index, p_hash.clone());
                insert_hash_to_content_in_diff(p_hash, &mut diff_hash_to_content);
            }
            EitherOrBoth::Right((n_index, n_hash)) => {
                diff_line_to_hash.insert(*n_index, n_hash.clone());
            }
            _ => {}
        }
    }


    HashedContent {
        line_to_hash: diff_line_to_hash,
        hash_to_content: diff_hash_to_content,
    }
}

pub fn file_to_lines(file_path: &Path) -> Vec<Line> {
    let file = File::open(file_path).expect("cannot open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| Line::new(line.unwrap()))
        .collect()
}

pub fn create_content_block(prev: &Path, new: &Path) {
    let prev_content = file_to_lines(prev);
    let new_content = file_to_lines(new);

    let mut final_content = ContentBlock::new();

    for pair in prev_content.iter().zip_longest(new_content.iter()) {
        match pair {
            EitherOrBoth::Both(prev_line, new_line) => {
                let diff_line = DiffLine::from(prev_line.clone(), new_line.clone());
                final_content.add(diff_line);
            }
            EitherOrBoth::Left(prev_DiffLine) => {
                let diff_line = DiffLine::from(prev_DiffLine.clone(), Line::new(String::new()));
                final_content.add(diff_line);
            }
            EitherOrBoth::Right(new_DiffLine) => {
                let diff_line = DiffLine::from(Line::new(String::new()), new_DiffLine.clone());
                final_content.add(diff_line);
            }
        }
    }

    let json = serde_json::to_string_pretty(&final_content).unwrap();
    println!("{}", json);
}
