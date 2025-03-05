use crate::add::{command_add};
use crate::init::init;
use crate::status::command_status;
use clap::{Parser, Subcommand};
use crate::commit_pointer::command_commit;
use crate::remote::{command_remote};
use crate::restore::command_restore;

/// A simple version control system built with Rust
#[derive(Parser)]
#[command(name = "Chak")]
#[command(about = "A simple version control system built with Rust", long_about = None)]
struct Args {
    /// The subcommand to run
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand, Clone)]
pub enum RemoteCommand {
    Add {
        remote: String,
        alias: String,
    },
    Remove {
        // #[arg(required = true)]
        // remote: String,
        alias: String,
    },
    Update {
        alias: String,
        remote: String,
    },
    // #[arg(required = true)]
    // remote: String,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initializes a new repository
    Init,
    /// Commits changes
    Commit {
        /// Commit message
        #[arg(short, long)]
        m: String,
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

    Restore {
        files: Vec<String>,
    },

    Remote {
        #[command(subcommand)]
        command: RemoteCommand,
    },
    // #[arg(required = true)]
    // alias: String,
}

pub fn parse_commandline() {
    let args = Args::parse();



    // Match against the commands
    match args.command {
        Some(Commands::Init {}) => {
            // Add logic for repository initialization
            init().expect("cant init the system");
        }

        Some(Commands::Add { files }) => {
            command_add(files);
        }
        Some(Commands::Commit { m }) => {
            command_commit(m);
        }
        Some(Commands::Restore { files }) => {
            command_restore(files)
        } ,
        Some(Commands::Status) => {
            command_status();
        }
        Some(Commands::Remote {command})  => command_remote(&command),
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
