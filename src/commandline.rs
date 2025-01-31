
use clap::{Parser, Subcommand};
use crate::init::init;
use crate::config::{get_current_dir};
use crate::status::get_status;
use crate::util::check_vcs_presence;

/// A simple version control system built with Rust
#[derive(Parser)]
#[command(name = "Chak")]
#[command(about = "A simple version control system built with Rust", long_about = None)]
struct Args {
    /// The subcommand to run
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes a new repository
    Init {
    },

    /// Commits changes
    Commit {
        /// Commit message
        #[arg(short, long)]
        message: String,

        /// Files to commit (use '.' for all files)
        #[arg(short, long, default_value = ".")]
        files: String,
    },

    Add {
        /// Files or directories to add
        #[arg(required = true)]
        files: Vec<String>,
    },
    /// Shows the status of the repository
    Status,

    /// Shows the commit history
    Log,

    /// Create and manage branches
    Branch {
        /// Create a new branch
        #[arg(short, long)]
        create: Option<String>,

        /// List all branches
        #[arg(short, long)]
        list: bool,
    },
}




pub fn parse_commandline() {
    let args = Args::parse();
    // Match against the commands
    match args.command {
        Some(Commands::Init { }) => {
            // Add logic for repository initialization
            init();
        }

        Some(Commands::Add {files }) => {
            if check_vcs_presence() {
               //
            }else {
                println!("No vcs_presence configured. could not applied add operations.");
            }
        }


        Some(Commands::Commit { message, files }) => {
            println!("Committing changes with message: '{}'", message);
            println!("Files to commit: {}", files);
            // Add commit logic here (e.g., staging and committing files)
            // commit_changes(&message, &files, &CURRENT_PATH);
        }
        Some(Commands::Status) => {
            if check_vcs_presence() {
                println!("\n{:?}",get_status(get_current_dir()) );
            }else {
                println!("No vcs_presence configured. could not applied add operations.");
            }
            // Add logic to display repository status
            //show_status(&CURRENT_PATH);
        }
        Some(Commands::Log) => {
            println!("Fetching commit log...");
            // Add logic to show the commit history
            // show_log( &CURRENT_PATH);
        }
        Some(Commands::Branch { create, list }) => {
            if let Some(branch_name) = create {
                println!("Creating branch '{}'", branch_name);
                // Add logic to create a new branch
                // create_branch(&branch_name, &CURRENT_PATH);
            } else if list {
                println!("Listing branches...");
                // Add logic to list all branches
                // list_branches( &CURRENT_PATH);
            } else {
                println!("Specify either --create or --list.");
            }
        }
        None => {
            println!("No command provided. Use --help for available commands.");
        }
    }
}

