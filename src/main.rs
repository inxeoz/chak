mod add;
mod commandline;
mod config;
mod custom_error;
mod hashed_algo;
mod hash_pointer_algo;
mod init;
mod tree_object;
mod util;
mod restore;
mod config_global;
mod status;
mod remote;
mod tree_hash_pointer;
mod hashed_blob;
mod version_hashed;
mod commit;
mod common;
mod versioning;
mod version_head;
mod hash_pointer;
mod renaming;
mod trait_extension;
//test


use clap::{Parser, Subcommand};
use crate::commandline::parse_commandline;

fn main() {
    parse_commandline();
}
