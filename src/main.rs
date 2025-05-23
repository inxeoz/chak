mod add;
mod commandline;
mod config;
mod custom_error;
mod init;
mod root_tree_object;
mod util;
mod restore;
mod config_global;
mod status;
mod remote;
mod root_tree_pointer;
mod blob_pointer;
mod version_object;
mod commit_pointer;
mod common;
mod version_head_object;
mod version_head_pointer;
mod hash_pointer;
mod renaming;
mod trait_extension;
mod nested_tree_pointer;
mod nested_tree_object;
mod blob_object;
mod version_pointer;
mod commit_object;
mod chak_traits;
mod trait_macros;
mod restricted;
mod takesnapshot;
mod handle_ignore;
mod stage;

use crate::commandline::parse_commandline;

fn main() {
    parse_commandline().expect("TODO: panic message");
}
