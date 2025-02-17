use std::process::Command;
use std::io::{self, Write}; // Import Write trait

pub fn main() -> io::Result<()> {
    // Check if the program is running as root (or has sudo privileges)
    if !is_root() {
        eprintln!("This program requires sudo privileges to create a directory in /opt/.");
        return Ok(()); // Exit gracefully if not root
    }

    // Construct the command to create the directory
    let output = Command::new("mkdir")
        .arg("-p") // Create parent directories if they don't exist
        .arg("/opt/raja")
        .status()?;  // Execute the command and handle errors

    match output.success() {
        true => {
            println!("Directory /opt/raja created successfully.");
            Ok(()) // Return Ok(()) to indicate success
        }
        false => {
            eprintln!("Failed to create directory /opt/raja.");
            io::stdout().flush()?; // Flush stdout to ensure message is printed
            std::process::exit(output.code().unwrap_or(1)); // Exit with error code
        }
    }
}

// Helper function to check if the program is running as root
fn is_root() -> bool {
    unsafe { libc::getuid() == 0 }
}

