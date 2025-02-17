use crate::config::get_project_dir;
use crate::util::check_vcs_presence;

pub fn command_status() {


    if check_vcs_presence(get_project_dir()) {
        // println!("\n{:?}",get_status(get_project_dir()) );
    } else {
        println!("No vcs_presence configured. could not applied add operations.");
    }
    //show_status(&CURRENT_PATH);
    // Add logic to display repository status

    println!("status");
}