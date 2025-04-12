
use crate::commandline::RemoteCommand;

pub fn command_remote(remote_command: &RemoteCommand) {

    match remote_command {
        RemoteCommand::Add { remote, alias } => {
        }
        RemoteCommand::Remove { .. } => {}
        RemoteCommand::Update { .. } => {}
    }

}

