use crate::core_logic::mcp::{self as core_mcp, MCPServer, MCPProjectConfig, AddServerResult, ImportResult, ServerStatus};
use std::collections::HashMap;
use tauri::{command, AppHandle};

#[command]
pub async fn mcp_add(
    app: AppHandle,
    name: String,
    transport: String,
    command: Option<String>,
    args: Vec<String>,
    env: HashMap<String, String>,
    url: Option<String>,
    scope: String,
) -> Result<AddServerResult, String> {
    core_mcp::add_server(&app, name, transport, command, args, env, url, scope)
}

#[command]
pub async fn mcp_list(app: AppHandle) -> Result<Vec<MCPServer>, String> {
    core_mcp::list_servers(&app)
}

#[command]
pub async fn mcp_get(app: AppHandle, name: String) -> Result<MCPServer, String> {
    core_mcp::get_server(&app, name)
}

#[command]
pub async fn mcp_remove(app: AppHandle, name: String) -> Result<String, String> {
    core_mcp::remove_server(&app, name)
}

#[command]
pub async fn mcp_add_json(
    app: AppHandle,
    name: String,
    json_config: String,
    scope: String,
) -> Result<AddServerResult, String> {
    core_mcp::add_server_from_json(&app, name, json_config, scope)
}

#[command]
pub async fn mcp_add_from_claude_desktop(
    app: AppHandle,
    scope: String,
) -> Result<ImportResult, String> {
    core_mcp::add_server_from_claude_desktop(app, scope).await
}

#[command]
pub async fn mcp_serve(app: AppHandle) -> Result<String, String> {
    core_mcp::serve(&app)
}

#[command]
pub async fn mcp_test_connection(app: AppHandle, name: String) -> Result<String, String> {
    core_mcp::test_connection(&app, name)
}

#[command]
pub async fn mcp_reset_project_choices(app: AppHandle) -> Result<String, String> {
    core_mcp::reset_project_choices(&app)
}

#[command]
pub async fn mcp_get_server_status() -> Result<HashMap<String, ServerStatus>, String> {
    core_mcp::get_server_status()
}

#[command]
pub async fn mcp_read_project_config(project_path: String) -> Result<MCPProjectConfig, String> {
    core_mcp::read_project_config(project_path)
}

#[command]
pub async fn mcp_save_project_config(
    project_path: String,
    config: MCPProjectConfig,
) -> Result<String, String> {
    core_mcp::save_project_config(project_path, config)
}