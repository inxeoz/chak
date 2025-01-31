
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

use clap::{Parser, Subcommand};
use crate::add::{ start_snapshot};
use crate::config::get_current_dir;
use crate::diff_algo::{compare_hashed_content, to_interconnected_line};
use crate::init::init;

fn main() {
  //  test();
    init();
    start_snapshot()
}


fn test() {


    // Convert the HashMap to a format suitable for serialization
    let file_path = &get_current_dir().join("file.txt");
    let file_path2 = &get_current_dir().join("file2.txt");

    // Generate mappings
    let first = to_interconnected_line(file_path);
    let second = to_interconnected_line(file_path2);

    // Serialize and print mappings
    println!("Line to Hash:");
    println!("{}", serde_json::to_string_pretty(&first.line_to_hash).unwrap());

    println!("Hash to Content:");
    println!("{}", serde_json::to_string_pretty(&first.hash_to_content).unwrap());

    // Serialize and print mappings
    println!("Line to Hash:");
    println!("{}", serde_json::to_string_pretty(&second.line_to_hash).unwrap());

    println!("Hash to Content:");
    println!("{}", serde_json::to_string_pretty(&second.hash_to_content).unwrap());

    let new = compare_hashed_content(first, second);

    // Serialize and print mappings
    println!("Line to Hash:");
    println!("{}", serde_json::to_string_pretty(&new.line_to_hash).unwrap());

    println!("Hash to Content:");
    println!("{}", serde_json::to_string_pretty(&new.hash_to_content).unwrap());



}
