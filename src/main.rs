use rusty_cli::command_handler::CommandHandlerArguments;
use rusty_cli::meta_data::ApplicationMetaData;
use rusty_cli::runner::Runner;
use crate::commands::list_versions::list_versions_command;
use crate::commands::set_command::set_command;


mod commands;
mod get_versions;

fn main() {

    let mut runner = Runner::new();
    runner.set_meta_data(ApplicationMetaData {
        title: "PHP version switcher".to_string(),
        description: "Makes switching your php version quite easy on macOS".to_string()
    });
    runner.enable_command_handler(CommandHandlerArguments{
        commands: vec![
            list_versions_command(),
            set_command()
        ],
        default_no_argument_callback: None,
        flags: vec![]
    });
    runner.run();


}
