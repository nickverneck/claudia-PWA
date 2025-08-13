use crate::core_logic::agents::{self as core_agents, Agent, AgentRun, AgentRunWithMetrics, AgentExport, GitHubAgentFile};
use tauri::{command, AppHandle, State};
use crate::commands::agents::AgentDb;

#[command]
pub async fn list_agents(db: State<'_, AgentDb>) -> Result<Vec<Agent>, String> {
    core_agents::list_agents(&db)
}

#[command]
pub async fn create_agent(
    db: State<'_, AgentDb>,
    name: String,
    icon: String,
    system_prompt: String,
    default_task: Option<String>,
    model: Option<String>,
    enable_file_read: Option<bool>,
    enable_file_write: Option<bool>,
    enable_network: Option<bool>,
    hooks: Option<String>,
) -> Result<Agent, String> {
    core_agents::create_agent(&db, name, icon, system_prompt, default_task, model, enable_file_read, enable_file_write, enable_network, hooks)
}

#[command]
pub async fn update_agent(
    db: State<'_, AgentDb>,
    id: i64,
    name: String,
    icon: String,
    system_prompt: String,
    default_task: Option<String>,
    model: Option<String>,
    enable_file_read: Option<bool>,
    enable_file_write: Option<bool>,
    enable_network: Option<bool>,
    hooks: Option<String>,
) -> Result<Agent, String> {
    core_agents::update_agent(&db, id, name, icon, system_prompt, default_task, model, enable_file_read, enable_file_write, enable_network, hooks)
}

#[command]
pub async fn delete_agent(db: State<'_, AgentDb>, id: i64) -> Result<(), String> {
    core_agents::delete_agent(&db, id)
}

#[command]
pub async fn get_agent(db: State<'_, AgentDb>, id: i64) -> Result<Agent, String> {
    core_agents::get_agent(&db, id)
}

#[command]
pub async fn list_agent_runs(
    db: State<'_, AgentDb>,
    agent_id: Option<i64>,
) -> Result<Vec<AgentRun>, String> {
    core_agents::list_agent_runs(&db, agent_id)
}

#[command]
pub async fn get_agent_run(db: State<'_, AgentDb>, id: i64) -> Result<AgentRun, String> {
    core_agents::get_agent_run(&db, id)
}

#[command]
pub async fn get_agent_run_with_real_time_metrics(
    db: State<'_, AgentDb>,
    id: i64,
) -> Result<AgentRunWithMetrics, String> {
    core_agents::get_agent_run_with_real_time_metrics(&db, id).await
}

#[command]
pub async fn list_agent_runs_with_metrics(
    db: State<'_, AgentDb>,
    agent_id: Option<i64>,
) -> Result<Vec<AgentRunWithMetrics>, String> {
    core_agents::list_agent_runs_with_metrics(&db, agent_id).await
}

#[command]
pub async fn execute_agent(
    app: AppHandle,
    agent_id: i64,
    project_path: String,
    task: String,
    model: Option<String>,
    db: State<'_, AgentDb>,
    registry: State<'_, crate::process::ProcessRegistryState>,
) -> Result<i64, String> {
    core_agents::execute_agent(app, agent_id, project_path, task, model, &db, &registry).await
}

#[command]
pub async fn list_running_sessions(
    db: State<'_, AgentDb>,
    registry: State<'_, crate::process::ProcessRegistryState>,
) -> Result<Vec<AgentRun>, String> {
    core_agents::list_running_sessions(&db, &registry).await
}

#[command]
pub async fn kill_agent_session(
    app: AppHandle,
    db: State<'_, AgentDb>,
    registry: State<'_, crate::process::ProcessRegistryState>,
    run_id: i64,
) -> Result<bool, String> {
    core_agents::kill_agent_session(app, &db, &registry, run_id).await
}

#[command]
pub async fn get_session_status(
    db: State<'_, AgentDb>,
    run_id: i64,
) -> Result<Option<String>, String> {
    core_agents::get_session_status(&db, run_id)
}

#[command]
pub async fn cleanup_finished_processes(db: State<'_, AgentDb>) -> Result<Vec<i64>, String> {
    core_agents::cleanup_finished_processes(&db)
}

#[command]
pub async fn get_live_session_output(
    registry: State<'_, crate::process::ProcessRegistryState>,
    run_id: i64,
) -> Result<String, String> {
    core_agents::get_live_session_output(&registry, run_id)
}

#[command]
pub async fn get_session_output(
    db: State<'_, AgentDb>,
    registry: State<'_, crate::process::ProcessRegistryState>,
    run_id: i64,
) -> Result<String, String> {
    core_agents::get_session_output(&db, &registry, run_id).await
}

#[command]
pub async fn stream_session_output(
    app: AppHandle,
    db: State<'_, AgentDb>,
    run_id: i64,
) -> Result<(), String> {
    core_agents::stream_session_output(app, &db, run_id).await
}

#[command]
pub async fn export_agent(db: State<'_, AgentDb>, id: i64) -> Result<String, String> {
    core_agents::export_agent(&db, id)
}

#[command]
pub async fn export_agent_to_file(
    db: State<'_, AgentDb>,
    id: i64,
    file_path: String,
) -> Result<(), String> {
    core_agents::export_agent_to_file(&db, id, file_path)
}

#[command]
pub async fn get_claude_binary_path(db: State<'_, AgentDb>) -> Result<Option<String>, String> {
    core_agents::get_claude_binary_path(&db)
}

#[command]
pub async fn set_claude_binary_path(db: State<'_, AgentDb>, path: String) -> Result<(), String> {
    core_agents::set_claude_binary_path(&db, path)
}

#[command]
pub async fn list_claude_installations(
    app: AppHandle,
) -> Result<Vec<crate::claude_binary::ClaudeInstallation>, String> {
    core_agents::list_claude_installations(&app)
}

#[command]
pub async fn import_agent(db: State<'_, AgentDb>, json_data: String) -> Result<Agent, String> {
    core_agents::import_agent(&db, json_data)
}

#[command]
pub async fn import_agent_from_file(
    db: State<'_, AgentDb>,
    file_path: String,
) -> Result<Agent, String> {
    core_agents::import_agent_from_file(&db, file_path).await
}

#[command]
pub async fn fetch_github_agents() -> Result<Vec<GitHubAgentFile>, String> {
    core_agents::fetch_github_agents().await
}

#[command]
pub async fn fetch_github_agent_content(download_url: String) -> Result<AgentExport, String> {
    core_agents::fetch_github_agent_content(download_url).await
}

#[command]
pub async fn import_agent_from_github(db: State<'_, AgentDb>, download_url: String) -> Result<Agent, String> {
    core_agents::import_agent_from_github(&db, download_url).await
}
