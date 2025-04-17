use crate::chak_traits::{HashPointerTraits, ObjectCommonTraits};

use crate::blob_object::BlobObject;
use crate::config::{
    CHAK_FOLDER_NAME, VCS_IGNORE_FILE_NAME, get_chak_fold_path, get_current_dir_path,
};
use crate::custom_error::ChakError;
use crate::handle_ignore::{handle_ignore_file, parse_ignore_combined_files_dirs};
use crate::hash_pointer::HashPointer;
use crate::root_tree_object::{NestedTreeObject, RootTreeObject};
use crate::stage::Stage;
use crate::util::{ path_buf_to_parent_and_name};
use ignore::gitignore::GitignoreBuilder;
use indexmap::IndexMap;
use std::path::{Path, PathBuf};

pub enum EntryType {
    FILE,
    DIR,
}


//when file hash have changed
pub struct OperationChanged {
    pub entry_old_hash: HashPointer,
    pub parent_dir_path: PathBuf,
    pub entry_name: String,
    pub entry_type: EntryType,
}

//when new file detected with new hash
pub struct OperationNew {
    pub parent_dir_path: PathBuf,
    pub new_entry_name: String,
    pub entry_type: EntryType,
}

//when entry exist in tree but got removed from directory , removed in original
pub struct OperationRemoved {
    pub entry_hash: HashPointer,
    pub removed_from: PathBuf,
    pub entry_name: String,
    pub entry_type: EntryType,
}

pub struct StatusDataStructure {
    pub changed_entry: IndexMap<HashPointer, OperationChanged>,
    pub new_entry: IndexMap<HashPointer, OperationNew>,
    pub removed_entry: IndexMap<HashPointer, OperationRemoved>,
}

pub fn command_status() -> Result<(), ChakError> {
    if get_chak_fold_path().exists() {
        
        let mut list_of_change = StatusDataStructure {
            changed_entry: IndexMap::new(),
            new_entry: IndexMap::new(),
            removed_entry: IndexMap::new(),
        };

        start_status(&mut list_of_change)?;

        list_of_change.removed_entry.iter().for_each( |(h, r)|
            println!("removed {} from {:?} ", r.entry_name, r.removed_from)
        );

        list_of_change.changed_entry.iter().for_each( | (h, v )|
            println!("changed {} ", v.entry_name)
        );

        list_of_change.new_entry.iter().for_each( |(h, v) |
        println!("new {}", v.new_entry_name)
        );



        if let Some(last_root_tree_pointer) = Stage::get_last_root_tree_pointer() {
            println!(
                "to be committed root pointer \n{}",
                last_root_tree_pointer.get_one_hash()
            );
        }
    } else {
        return Err(ChakError::RepoNotInitialized);
    }
    //show_status(&CURRENT_PATH);
    // Add logic to display repository status

    println!("status");
    Ok(())
}

//lets see working with vcs config
pub fn start_status(list_of_change: &mut StatusDataStructure) -> Result<(), ChakError> {
    let dir = get_current_dir_path();
    let mut ignore_builder = GitignoreBuilder::new(get_current_dir_path());

    let mut root_tree = RootTreeObject::get_root_object()
        .ok()
        .and_then(|v| v.accumulated_hash_pointer().ok());

    handle_ignore_file(
        &mut ignore_builder,
        vec![(Some(dir.to_path_buf()), CHAK_FOLDER_NAME)],
    );

    recursive_status(
        root_tree,
        dir,
        &mut ignore_builder,
        list_of_change,
    )?;
    Ok(())
}


pub fn recursive_status(
    tree_hash_pointer_opt: Option<HashPointer>,
    dir: &Path,
    ignore_builder: &mut GitignoreBuilder,
    list_of_change: &mut StatusDataStructure,
) -> Result<(), ChakError> {
    ignore_builder.add(dir.join(VCS_IGNORE_FILE_NAME)); // when work with nested ignore file or not

    let allowed_entries = parse_ignore_combined_files_dirs(dir, ignore_builder)?;


    if let Some(tree_pointer) = tree_hash_pointer_opt {

        let nested_tree = NestedTreeObject::from(&tree_pointer).ok();

        for entry in allowed_entries {
            let (parent_of_entry, entry_name) = path_buf_to_parent_and_name(&entry)?;
            let previous_hash_pointer_opt = get_entry_hash_from_tree(&entry, &entry_name, nested_tree.clone());

            if entry.is_file() {
                update_status_structure_for_file(
                    &entry,
                    previous_hash_pointer_opt,
                    &parent_of_entry,
                    entry_name,
                    list_of_change,
                )?;
            }else {

                recursive_status(
                    previous_hash_pointer_opt,
                    &entry,
                    ignore_builder,
                    list_of_change,
                )?
            }
        }
    }

    Ok(())
}



pub fn update_status_structure_for_file(
    entry: &Path,
    previous_hash_pointer_opt: Option<HashPointer>,
    parent_of_entry: &Path,
    entry_name: String,
    list_of_change: &mut StatusDataStructure,
) -> Result<(), ChakError> {
    let new_hash_pointer_file = BlobObject::from_file_path(&entry).accumulated_hash_pointer()?;

    if let Some(previous_hash_pointer) = previous_hash_pointer_opt {

        if previous_hash_pointer != new_hash_pointer_file {
            list_of_change.changed_entry.insert(
                new_hash_pointer_file,
                OperationChanged {
                    entry_old_hash: previous_hash_pointer,
                    parent_dir_path: parent_of_entry.to_path_buf(),
                    entry_name: entry_name.clone(),
                    entry_type: EntryType::FILE,
                },
            );
        }

    }else {
        //if there is not previous hash pointer does it mean new ?

        list_of_change.new_entry.insert(
            new_hash_pointer_file,
            OperationNew {
                parent_dir_path: parent_of_entry.to_path_buf(),
                entry_type: EntryType::FILE,
                new_entry_name: entry_name.clone(),
            },
        );


    }



    Ok(())
}

pub fn get_entry_hash_from_tree(
    entry: &Path,
    entry_name: &String,
    nested_tree_opt: Option<NestedTreeObject>,
) -> Option<HashPointer> {

    if let Some(nested_tree) = nested_tree_opt {

        let entry_hash_from_tree = if entry.is_file() {
            nested_tree.file_children.get(entry_name.as_str()).map(|v| {
                v.load_version_head()
                    .get_pointer_to_blob()
                    .projection_to_hash_pointer()
            })
        } else {
            nested_tree
                .dir_children
                .get(entry_name.as_str())
                .map(|d| d.projection_to_hash_pointer())
        };

        entry_hash_from_tree

    }else {
        None
    }
}


