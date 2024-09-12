use crate::utils::types::{HelpInfoType};

use commands::COMMANDS;
mod commands;

pub fn list_command() -> HelpInfoType {
    COMMANDS
}

// pub fn list_command_names() -> Vec<String> {
//     COMMANDS.iter().map(|x| x.command_name.to_string()).collect()
// }

// pub fn list_command_names_descriptions() -> Vec<(String, String)> {
//     COMMANDS.iter().map(|x| (x.command_name.to_string(), x.command_instructions.to_string())).collect()
// }