use crate::config::{get_current_dir, vcs_fold};


pub fn command_status() {
    if vcs_fold().exists() {
        // println!("\n{:?}",get_status(get_current_dir()) );
    } else {
        println!("No vcs_presence configured. could not applied add operations.");
    }
    //show_status(&CURRENT_PATH);
    // Add logic to display repository status

    println!("status");
}


pub fn check_previous_entry_status() {

}