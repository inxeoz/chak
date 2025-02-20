
use std::collections::HashMap;
use indexmap::IndexSet;
use crate::hashing::{hash_from_content, HashPointer, HashPointerTraits, VersionHashPointer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Version {
    version_number: u32,
    version_type: VersionType,
    diff_from: u32,
    hash_pointer: HashPointer,
}
#[derive(Serialize, Deserialize, Debug)]
enum VersionType {
    DIFF,
    FILE,
}


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
    pub hash_lines: IndexSet<String>,
    pub hash_to_content: HashMap<String, String>,
}


pub fn hashed_content_from_string_lines(lines: Vec<String>) -> HashedContent {
    let mut hash_lines = IndexSet::<String>::new();
    let mut hash_to_content = HashMap::<String, String>::new();
    for line in lines {
        let hash_line = hash_from_content::<HashPointer>(&line).get_one_hash();
        hash_lines.insert(hash_line.clone());
        hash_to_content.insert(hash_line, line);
    }
    HashedContent {
        hash_lines,
        hash_to_content,
    }
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct HashedContentForVersion {
    pub pointer_to_previous_version: Option<VersionHashPointer>,
    pub hashed_content: HashedContent,
}
impl HashedContentForVersion {
    pub fn new(content: HashedContent, pointer_to_previous_version: Option<VersionHashPointer>) -> Self {
        Self {
            pointer_to_previous_version,
            hashed_content: content,
        }
    }
}




