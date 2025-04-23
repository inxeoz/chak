use crate::config::{VCS_IGNORE_FILE_NAME, get_current_dir_path};
use crate::custom_error::ChakError;
use crate::root_tree_object::{ RootTreeObject};
use crate::root_tree_pointer::RootTreePointer;

use crate::util::path_buf_to_name;
use ignore::Match;
use ignore::gitignore::GitignoreBuilder;
use std::path::PathBuf;
use crate::nested_tree_object::NestedTreeObject;

pub fn start_individual_snapshot(entry_string: String) -> Result<(), ChakError> {
    let index = 0usize;

    let entry = PathBuf::from(entry_string);

    // Split all components
    let parts: Vec<String> = entry
        .components()
        .map(|comp| comp.as_os_str().to_string_lossy().into_owned())
        .collect();

    let ignore_builder = GitignoreBuilder::new(get_current_dir_path());
    let mut path = get_current_dir_path().to_owned();

    //get latest tree pointer from history_log
    let mut root_tree = RootTreeObject::get_root_object()
        .unwrap_or(RootTreeObject::new())
        .as_nested_tree();

    take_snapshot(ignore_builder, &mut path, parts, index, &mut root_tree)?;

    let new_root_tree_pointer = RootTreePointer::save_tree(&mut RootTreeObject::from(root_tree))?;
    //attaching the updated new tree pointer to stage temporarily because tree pointer can be changed util its commited
    new_root_tree_pointer.attach_tree_to_stage();

    Ok(())
}

pub fn take_snapshot(
    mut ignore_builder: GitignoreBuilder,
    path: &mut PathBuf,
    parts: Vec<String>,
    index: usize,
    tree: &mut NestedTreeObject,
) -> Result<(), ChakError> {
    if !(index < parts.len()) {
        return Ok(());
    }

    ignore_builder.add(path.join(VCS_IGNORE_FILE_NAME));

    if let Ok(ignore_build) = ignore_builder.build() {
        match ignore_build.matched(
            path.join(&parts[index]).to_str().unwrap_or(""),
            path.join(&parts[index]).is_dir(),
        ) {
            // can i use "#" for default
            Match::None => handle_matched_none(ignore_builder, path, parts, index, tree)?,

            Match::Ignore(matched_rule) => {
                println!("Ignored: {}", path.display());
                println!(
                    "Ignored due to rule: '{}' in file: {}",
                    matched_rule.original(),
                    matched_rule.from().unwrap().display()
                );
            }
            Match::Whitelist(_) => {
                //i have to work for it
            }
        }
    }

    Ok(())
}

fn handle_matched_none(
    ignore_builder: GitignoreBuilder,
    path: &mut PathBuf,
    parts: Vec<String>,
    index: usize,
    tree: &mut NestedTreeObject,
) -> Result<(), ChakError> {
    path.push(&parts[index]);

    if !path.exists() {
        println!("{:?} does not exist", path);
        return Ok(());
    }
    println!("starting adding {} ", &parts[index]);

    let path_name = path_buf_to_name(&path)?;

    if path.is_file() {
        tree.add_file_child(&path, path_name.as_str())?;
    } else {
        
        let existing_child_tree = tree.dir_children.get_mut(&path_name);
        let mut dir_nested_tree_object =
            existing_child_tree.map_or_else(|| NestedTreeObject::new(), |v| v.load_tree());

        take_snapshot(
            ignore_builder,
            path,
            parts,
            index + 1,
            &mut dir_nested_tree_object,
        )?;
        tree.add_dir_child(path_name, &mut dir_nested_tree_object)?;
    }

    Ok(())
}
