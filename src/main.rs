
mod init;
mod add;
mod macros;
mod util;
mod config;
mod commandline;
mod hashing;
mod diff;
mod diff_algo;
mod tree_object;
mod commit;
mod custom_error;

use std::fs::File;
use std::io;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use crate::commandline::parse_commandline;
use crate::config::{blob_fold, get_project_dir};
use crate::diff_algo::{compare_hashed_content, restore_previous_version, to_hashed_content};


fn main() {
    parse_commandline();
    //test();
   // restore_test()
}


fn test() -> io::Result<()>{


    // Convert the HashMap to a format suitable for serialization
    let file1 = File::open(&get_project_dir().join("file1.txt"))?;
    let file2 = File::open(&get_project_dir().join("file2.txt"))?;

    // Generate mappings
    let first = to_hashed_content(&file1);
    let second = to_hashed_content(&file2);

    // Serialize and print mappings
    println!("hash lines:");
    println!("{}", serde_json::to_string_pretty(&first.hash_lines)?);

    println!("Hash to Content:");
    println!("{}", serde_json::to_string_pretty(&first.hash_to_content)?);

    // Serialize and print mappings
    println!("hash lines:");
    println!("{}", serde_json::to_string_pretty(&second.hash_lines)?);

    println!("Hash to Content:");
    println!("{}", serde_json::to_string_pretty(&second.hash_to_content)?);

    let new = compare_hashed_content(first, second);

    // Serialize and print mappings
    println!("hash lines:");
    println!("{}", serde_json::to_string_pretty(&new.hash_lines)?);

    println!("Hash to Content:");
    println!("{}", serde_json::to_string_pretty(&new.hash_to_content)?);
    Ok(())



}


fn restore_test(){

    let prev_file =PathBuf::from("file1.txt");
    let new_file = PathBuf::from("file2.txt");

   // restore_previous_version(&blob_path, &diff_path);


}