use std::path::{Path, PathBuf};
use ignore::gitignore::GitignoreBuilder;
use libc::iovec;
use crate::blob_object::BlobObject;
use crate::blob_pointer::BlobObjectPointer;
use crate::chak_traits::HashPointerTraits;
use crate::config::{blob_fold, get_current_dir, vcs_fold, VCS_FOLDER, VCS_IGNORE_FILE};
use crate::custom_error::ChakError;
use crate::handle_ignore::{handle_ignore_file, parse_ignore};
use crate::hash_pointer::HashPointer;
use crate::object::ObjectTraits;

pub fn command_status() -> Result<(), ChakError> {

    if vcs_fold().exists() {
        println!();
        let mut list_of_change = Vec::<String>::new();
        start_status(&mut list_of_change)?;

        if ! list_of_change.is_empty() {
            println!("files to track ");
            for file in list_of_change {
                println!("{}", file);
            }
        }else {
            println!("no files to track");
        }

    } else {
        println!("No vcs_presence configured. can't get status");
        return Ok(())
    }
    //show_status(&CURRENT_PATH);
    // Add logic to display repository status

    println!("status");
    return Ok(())
}

//lets see working with vcs config
pub fn start_status(list_of_change: &mut Vec<String>) -> Result<(), ChakError> {
    let dir = get_current_dir();
    let mut ignore_builder = GitignoreBuilder::new(get_current_dir());

    handle_ignore_file(
        &mut ignore_builder,
        vec![(Some(dir.to_path_buf()), VCS_FOLDER)],
    );


    recursive_status(dir, &mut ignore_builder,  list_of_change)?;
    return Ok(())
}

pub fn recursive_status(dir: &Path, ignore_builder: &mut GitignoreBuilder, list_of_change: &mut Vec<String>) -> Result<(), ChakError> {

    ignore_builder.add(dir.join(VCS_IGNORE_FILE));

    let (allowed_file_entries, allowed_dir_entries) = parse_ignore(dir, ignore_builder)?;

    for entry in allowed_file_entries {
           let hash_pointer = HashPointer::from_file_path(&entry)?;
          if ! BlobObject::containing_folder().join(hash_pointer.get_path()).exists() {
              list_of_change.push(entry.to_str().unwrap().to_string());
          }
    }
    for entry in allowed_dir_entries {
        recursive_status(&entry, ignore_builder, list_of_change)?;
    }
    return  Ok(())

}