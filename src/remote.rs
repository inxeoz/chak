use clap::Subcommand;

#[derive(Debug, Subcommand)]
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

pub fn command_remote(remote_command: &RemoteCommand) {

    match remote_command {
        RemoteCommand::Add { remote, alias } => {
        }
        RemoteCommand::Remove { .. } => {}
        RemoteCommand::Update { .. } => {}
    }

}