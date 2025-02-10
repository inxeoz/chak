
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
    let file_path = &get_project_dir().join("file.txt");
    let file_path2 = &get_project_dir().join("file2.txt");

    // Generate mappings
    let first = to_hashed_content(file_path)?;
    let second = to_hashed_content(file_path2)?;

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

    let blob_path =PathBuf::from("example/.chak/store/blobs/d2/1cb45e6aa32737784b46fc29a8af38e649ba96d6472f7ca558b2565639e349");
    let diff_path = PathBuf::from("example/.chak/store/blobs/1c/621c216a0090a4381722d184f94f651508b42ee422b9116a0a68ef19251613");
    restore_previous_version(&blob_path, &diff_path);


}