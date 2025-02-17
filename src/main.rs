mod add;
mod commandline;
mod commit;
mod config;
mod custom_error;
mod diff;
mod diff_algo;
mod hashing;
mod init;
mod tree_object;
mod util;
mod restore;
mod temporary;
mod test;
mod global_config;
mod status;
mod remote;

use crate::diff_algo::{
    compare_hashed_content, hashed_content_from_file,
};
use clap::{Parser, Subcommand};
use std::fs::File;
use std::io;
use std::path::PathBuf;
use crate::commandline::parse_commandline;
use crate::config::Config;
use crate::global_config::GlobalConfig;

fn main() {
    
    test::main().expect("TODO: panic message");
    parse_commandline();

//     let output = Command::new("mkdir") //  On Windows, you might need "cmd" /c mkdir raja
//         .arg("raja");
//         // .output()
//         // .expect("Failed to execute mkdir command");
//     //
//     // if output.status.success() {
//     //     println!("Directory  created successfully.");
//     // } else {
//     //     let stderr = String::from_utf8_lossy(&output.stderr);
//     //     eprintln!("Error creating directory {}", stderr);
//     // }
// let config = Config {
//     vcs_folder: "ignore".to_string(),
//     vcs_ignore_file: "ggg".to_string(),
//     min_hash_length: 1,
//     work_with_nested_ignore_file: true,
//     alias: vec![*output]
//
// };
//
//    let s =  serialize_struct(&config).as_str();
//
// save_or_create_file(&get_project_dir().join("cmd"), Some(s),  false, None);
   // util::tests::test_save_or_create();
    //test();
    // restore_test()
}

// fn test() -> io::Result<()> {
//     // Convert the HashMap to a format suitable for serialization
//     let file1 = File::open(&get_project_dir().join("file1.txt"))?;
//     let file2 = File::open(&get_project_dir().join("file2.txt"))?;
//
//     // Generate mappings
//     let first = hashed_content_from_file(&file1);
//     let second = hashed_content_from_file(&file2);
//
//     // Serialize and print mappings
//     println!("hash lines:");
//     println!("{}", toml::to_string_pretty(&first.hash_lines)?);
//
//     println!("Hash to Content:");
//     println!("{}", toml::to_string_pretty(&first.hash_to_content)?);
//
//     // Serialize and print mappings
//     println!("hash lines:");
//     println!("{}", toml::to_string_pretty(&second.hash_lines)?);
//
//     println!("Hash to Content:");
//     println!("{}", toml::to_string_pretty(&second.hash_to_content)?);
//
//     let new = compare_hashed_content(&first, &second);
//
//     // Serialize and print mappings
//     println!("hash lines:");
//     println!("{}", toml::to_string_pretty(&new.hash_lines)?);
//
//     println!("Hash to Content:");
//     println!("{}", toml::to_string_pretty(&new.hash_to_content)?);
//     Ok(())
// }

fn restore_test() {
    let prev_file = PathBuf::from("file1.txt");
    let new_file = PathBuf::from("file2.txt");

    // restore_previous_version(&blob_path, &diff_path);
}
