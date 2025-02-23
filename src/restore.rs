use crate::config::{get_project_dir};
use crate::util::{
    save_or_create_file,
};
use std::fs::{create_dir_all,};
use std::path::PathBuf;
use std::{ io};
use crate::object_pointer::ObjectPointer;
use crate::tree_object::{TreeHashPointer};

fn start_restoring(tree_root_pointer: TreeHashPointer, dir_path: &PathBuf) -> io::Result<()> {
    let tree_object = tree_root_pointer.load_tree();
   for (child_name, child_pointer) in tree_object.children {
       let actual_child_path = dir_path.join(PathBuf::from(child_name)); //in working folder
       match child_pointer {
           ObjectPointer::VersionHeadFile(vh) => {
               let hashed_content =vh.load_version_head().get_pointer_to_blob().load_blob();
               let content =hashed_content.to_string_content();
               //save blob data into actual child
               save_or_create_file(&actual_child_path, Some(&content), false, None)?;
           }
           ObjectPointer::TreeFIle(t) => {
               if ! actual_child_path.exists() {
                   create_dir_all(actual_child_path.clone())?;
               }
               start_restoring(t.clone(), &actual_child_path)?;
           }
       }

   }
    Ok(())
}
pub fn command_restore(files: Vec<String>) {
    if files.contains(&".".to_string()) {
        if let Some(latest_tree_pointer)  = TreeHashPointer::get_latest_tree_root_pointer(true) {
            start_restoring(latest_tree_pointer, get_project_dir()).expect("TODO: panic message");
        }
    }
}
