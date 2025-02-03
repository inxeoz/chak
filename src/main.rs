
mod init;
mod add;
mod macros;
mod util;
mod config;
mod commandline;
mod status;
mod hashing;
mod diff;
mod diff_algo;
mod tree_object;
mod commit;
mod custom_error;

use std::io;
use clap::{Parser, Subcommand};
use crate::commandline::parse_commandline;
use crate::config::get_project_dir;
use crate::diff_algo::{compare_hashed_content, to_interconnected_line};


fn main() {
    parse_commandline();
}


fn test() -> io::Result<()>{


    // Convert the HashMap to a format suitable for serialization
    let file_path = &get_project_dir().join("file.txt");
    let file_path2 = &get_project_dir().join("file2.txt");

    // Generate mappings
    let first = to_interconnected_line(file_path)?;
    let second = to_interconnected_line(file_path2)?;

    // Serialize and print mappings
    println!("Line to Hash:");
    println!("{}", serde_json::to_string_pretty(&first.line_to_hash)?);

    println!("Hash to Content:");
    println!("{}", serde_json::to_string_pretty(&first.hash_to_content)?);

    // Serialize and print mappings
    println!("Line to Hash:");
    println!("{}", serde_json::to_string_pretty(&second.line_to_hash)?);

    println!("Hash to Content:");
    println!("{}", serde_json::to_string_pretty(&second.hash_to_content)?);

    let new = compare_hashed_content(first, second);

    // Serialize and print mappings
    println!("Line to Hash:");
    println!("{}", serde_json::to_string_pretty(&new.line_to_hash)?);

    println!("Hash to Content:");
    println!("{}", serde_json::to_string_pretty(&new.hash_to_content)?);
    Ok(())



}
