use std::fmt::format;
use std::{fs, str, string};
use std::fs::{File, OpenOptions};
use std::path::Path;
use rusty_cli::commands::command::Command;
use rusty_cli::file_reader::FileReader;
use rusty_cli::flags::flag::Flags;
use rusty_cli::inputs::select_input::SelectInput;
use crate::get_versions::{get_current_version, get_versions};
use std::io::Write;
use std::process::Stdio;
use std::ptr::replace;
use crate::linking::{link_php_version, unlink_php_version};
use crate::replacer::replace_php_version;

fn executor(_flags: Flags) {
    let versions = get_versions();
    let selected_version = SelectInput::get_value(versions);

    println!("Fetching current php version...");
    let current_version = get_current_version();
    println!("Current php version is {}", current_version.clone());

    println!("Setting php version to {}", &selected_version);

    println!("Unlinking current php version...");
    unlink_php_version(current_version.clone());

    println!("Linking php version {}", selected_version.clone());
    link_php_version(selected_version.clone());

    let zsh_path = format!("/Users/{}/.zshrc", whoami::username());
    if !Path::new(zsh_path.clone().as_str()).exists() {
        println!("Other shells then zsh are not supported yet");
        // TODO: Implement support for more shells
        return;
    }

    println!("Setting shell configuration...");
    replace_php_version(zsh_path, current_version, selected_version.clone());

    println!("Successfully switched to php version {}", selected_version);
}

/// The command that sets the php version of the system
pub fn set_command() -> Command {
    Command::new(
        "Set command".to_string(),
        "Sets the current php version".to_string(),
        "php-version-switcher set".to_string(),
        executor,
        "set".to_string()
    )
}