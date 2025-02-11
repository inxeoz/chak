
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

use indexmap::IndexSet;

fn main2() {
    // Creating IndexSets
    let mut set1 = IndexSet::new();
    set1.insert("apple");
    set1.insert("banana");
    set1.insert("cherry");

    let mut set2 = IndexSet::new();
    set2.insert("banana");
    set2.insert("grape");
    set2.insert("orange");

    println!("Set 1: {:?}", set1); // Output: Set 1: {"apple", "banana", "cherry"}
    println!("Set 2: {:?}", set2); // Output: Set 2: {"banana", "grape", "orange"}

    // 1. Union: Combining elements (no duplicates)
    let union_set: IndexSet<_> = set1.union(&set2).cloned().collect();
    println!("Union: {:?}", union_set); // Output: Union: {"apple", "banana", "cherry", "grape", "orange"}

    // 2. Intersection: Common elements
    let intersection_set: IndexSet<_> = set1.intersection(&set2).cloned().collect();
    println!("Intersection: {:?}", intersection_set); // Output: Intersection: {"banana"}

    // 3. Difference (Subtraction): Elements in set1 but not in set2
    let difference_set: IndexSet<_> = set1.difference(&set2).cloned().collect(); // set1 - set2
    println!("Difference (set1 - set2): {:?}", difference_set); // Output: Difference (set1 - set2): {"apple", "cherry"}

    // 4. Difference (Subtraction): Elements in set2 but not in set1
    let difference_set2: IndexSet<_> = set2.difference(&set1).cloned().collect(); // set2 - set1
    println!("Difference (set2 - set1): {:?}", difference_set2); // Output: Difference (set2 - set1): {"grape", "orange"}


    // 5. Symmetric Difference: Elements in either set, but not both
    let symmetric_difference_set: IndexSet<_> = set1.symmetric_difference(&set2).cloned().collect();
    println!("Symmetric Difference: {:?}", symmetric_difference_set); // Output: Symmetric Difference: {"apple", "cherry", "grape", "orange"}

    // 6. Subset and Superset checks
    println!("Is set1 a subset of set2? {}", set1.is_subset(&set2)); // Output: Is set1 a subset of set2? false
    println!("Is set1 a superset of set2? {}", set1.is_superset(&set2)); // Output: Is set1 a superset of set2? false

    // 7. Adding and removing elements
    set1.insert("date");
    println!("Set 1 after insert: {:?}", set1); // Output: Set 1 after insert: {"apple", "banana", "cherry", "date"}

    set1.remove("banana");
    println!("Set 1 after remove: {:?}", set1); // Output: Set 1 after remove: {"apple", "cherry", "date"}

    // 8. Clearing the set
    // set1.clear();
    // println!("Set 1 after clear: {:?}", set1); // Output: Set 1 after clear: {}

}