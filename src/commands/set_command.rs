use rusty_cli::commands::command::Command;
use rusty_cli::file_reader::FileReader;
use rusty_cli::flags::flag::Flags;
use rusty_cli::inputs::select_input::SelectInput;
use crate::get_versions::get_versions;

fn executor(_flags: Flags) {
    let versions = get_versions();
    let selected_value = SelectInput::get_value(versions);
    println!("Setting php version to {}", &selected_value);
    for version in versions.clone() {
        std::process::Command::new("brew")
            .args("unlink")
            .args(format!("php@{}", version))
            .output()
            .expect("Cannot unlink installed php version");
    }

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