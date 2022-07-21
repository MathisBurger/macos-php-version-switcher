use std::fs;
use rusty_cli::commands::command::Command;
use rusty_cli::flags::flag::Flags;
use crate::get_versions::get_versions;

/// Gets all installed php versions by the path
/// and prints them to the standard output
fn executor(_flags: Flags) {
    println!("These are the current installed php versions:");
    for version in get_versions() {
        println!("{}", version);
    }
}

/// Command that is used to list all current php versions
/// that are installed
pub fn list_versions_command() -> Command {
    Command::new(
        "List versions".to_string(),
            "Lists all installed versions of php".to_string(),
        "php-version-switcher list".to_string(),
        executor,
        "list".to_string()
    )
}