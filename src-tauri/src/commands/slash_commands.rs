use crate::core_logic::slash_commands::{self as core_slash_commands, SlashCommand};
use tauri::command;

#[command]
pub async fn slash_commands_list(
    project_path: Option<String>,
) -> Result<Vec<SlashCommand>, String> {
    core_slash_commands::list_commands(project_path)
}

#[command]
pub async fn slash_command_get(command_id: String) -> Result<SlashCommand, String> {
    core_slash_commands::get_command(command_id)
}

#[command]
pub async fn slash_command_save(
    scope: String,
    name: String,
    namespace: Option<String>,
    content: String,
    description: Option<String>,
    allowed_tools: Vec<String>,
    project_path: Option<String>,
) -> Result<SlashCommand, String> {
    core_slash_commands::save_command(scope, name, namespace, content, description, allowed_tools, project_path)
}

#[command]
pub async fn slash_command_delete(command_id: String, project_path: Option<String>) -> Result<String, String> {
    core_slash_commands::delete_command(command_id, project_path)
}