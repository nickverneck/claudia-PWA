#![cfg(feature = "web")]

use axum::{
    routing::{get, post},
    Router,
    Json,
    extract::{Query, State},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

use claudia_lib::core_logic::usage::{self as core_usage, UsageStats, UsageEntry, ProjectUsage};
use claudia_lib::core_logic::storage::{self as core_storage, TableInfo, TableData, QueryResult};
use claudia_lib::core_logic::slash_commands::{self as core_slash_commands, SlashCommand};
use claudia_lib::core_logic::proxy::{self as core_proxy, ProxySettings};
use claudia_lib::core_logic::mcp::{self as core_mcp, MCPServer, MCPProjectConfig, AddServerResult, ImportResult, ServerStatus};
use claudia_lib::core_logic::claude::{self as core_claude, ClaudeSettings, ClaudeVersionStatus, ClaudeMdFile, FileEntry, Project, Session};
use claudia_lib::core_logic::agents::{self as core_agents, Agent, AgentRun, AgentRunWithMetrics, AgentExport, GitHubAgentFile};
use claudia_lib::commands::agents::{AgentDb, init_database_web};

#[derive(Deserialize)]
struct UsageStatsParams {
    days: Option<u32>,
}

async fn get_usage_stats(Query(params): Query<UsageStatsParams>) -> Json<UsageStats> {
    Json(core_usage::get_usage_stats(params.days).unwrap())
}

#[derive(Deserialize)]
struct UsageByDateRangeParams {
    start_date: String,
    end_date: String,
}

async fn get_usage_by_date_range(Query(params): Query<UsageByDateRangeParams>) -> Json<UsageStats> {
    Json(core_usage::get_usage_by_date_range(params.start_date, params.end_date).unwrap())
}

#[derive(Deserialize)]
struct UsageDetailsParams {
    project_path: Option<String>,
    date: Option<String>,
}

async fn get_usage_details(Query(params): Query<UsageDetailsParams>) -> Json<Vec<UsageEntry>> {
    Json(core_usage::get_usage_details(params.project_path, params.date).unwrap())
}

#[derive(Deserialize)]
struct SessionStatsParams {
    since: Option<String>,
    until: Option<String>,
    order: Option<String>,
}

async fn get_session_stats(Query(params): Query<SessionStatsParams>) -> Json<Vec<ProjectUsage>> {
    Json(core_usage::get_session_stats(params.since, params.until, params.order).unwrap())
}

async fn storage_list_tables(State(db): State<Arc<Mutex<AgentDb>>>) -> Json<Vec<TableInfo>> {
    let db_guard = db.lock().unwrap();
    Json(core_storage::list_tables(&db_guard).unwrap())
}

#[derive(Deserialize)]
struct StorageReadTableParams {
    tableName: String,
    page: i64,
    pageSize: i64,
    searchQuery: Option<String>,
}

async fn storage_read_table(State(db): State<Arc<Mutex<AgentDb>>>, Query(params): Query<StorageReadTableParams>) -> Json<TableData> {
    let db_guard = db.lock().unwrap();
    Json(core_storage::read_table(&db_guard, params.tableName, params.page, params.pageSize, params.searchQuery).unwrap())
}

#[derive(Deserialize)]
struct StorageUpdateRowPayload {
    tableName: String,
    primaryKeyValues: std::collections::HashMap<String, serde_json::Value>,
    updates: std::collections::HashMap<String, serde_json::Value>,
}

async fn storage_update_row(State(db): State<Arc<Mutex<AgentDb>>>, Json(payload): Json<StorageUpdateRowPayload>) {
    let db_guard = db.lock().unwrap();
    core_storage::update_row(&db_guard, payload.tableName, payload.primaryKeyValues, payload.updates).unwrap();
}

#[derive(Deserialize)]
struct StorageDeleteRowPayload {
    tableName: String,
    primaryKeyValues: std::collections::HashMap<String, serde_json::Value>,
}

async fn storage_delete_row(State(db): State<Arc<Mutex<AgentDb>>>, Json(payload): Json<StorageDeleteRowPayload>) {
    let db_guard = db.lock().unwrap();
    core_storage::delete_row(&db_guard, payload.tableName, payload.primaryKeyValues).unwrap();
}

#[derive(Deserialize)]
struct StorageInsertRowPayload {
    tableName: String,
    values: std::collections::HashMap<String, serde_json::Value>,
}

async fn storage_insert_row(State(db): State<Arc<Mutex<AgentDb>>>, Json(payload): Json<StorageInsertRowPayload>) -> Json<i64> {
    let db_guard = db.lock().unwrap();
    Json(core_storage::insert_row(&db_guard, payload.tableName, payload.values).unwrap())
}

#[derive(Deserialize)]
struct StorageExecuteSqlPayload {
    query: String,
}

async fn storage_execute_sql(State(db): State<Arc<Mutex<AgentDb>>>, Json(payload): Json<StorageExecuteSqlPayload>) -> Json<QueryResult> {
    let db_guard = db.lock().unwrap();
    Json(core_storage::execute_sql(&db_guard, payload.query).unwrap())
}

#[derive(Deserialize)]
struct SlashCommandsListParams {
    project_path: Option<String>,
}

async fn slash_commands_list(Query(params): Query<SlashCommandsListParams>) -> Json<Vec<SlashCommand>> {
    Json(core_slash_commands::list_commands(params.project_path).unwrap())
}

#[derive(Deserialize)]
struct SlashCommandGetParams {
    command_id: String,
}

async fn slash_command_get(Query(params): Query<SlashCommandGetParams>) -> Json<SlashCommand> {
    Json(core_slash_commands::get_command(params.command_id).unwrap())
}

#[derive(Deserialize)]
struct SlashCommandSavePayload {
    scope: String,
    name: String,
    namespace: Option<String>,
    content: String,
    description: Option<String>,
    allowed_tools: Vec<String>,
    project_path: Option<String>,
}

async fn slash_command_save(Json(payload): Json<SlashCommandSavePayload>) -> Json<SlashCommand> {
    Json(core_slash_commands::save_command(payload.scope, payload.name, payload.namespace, payload.content, payload.description, payload.allowed_tools, payload.project_path).unwrap())
}

#[derive(Deserialize)]
struct SlashCommandDeletePayload {
    command_id: String,
    project_path: Option<String>,
}

async fn slash_command_delete(Json(payload): Json<SlashCommandDeletePayload>) -> Json<String> {
    Json(core_slash_commands::delete_command(payload.command_id, payload.project_path).unwrap())
}

async fn get_proxy_settings(State(db): State<Arc<Mutex<AgentDb>>>) -> Json<ProxySettings> {
    let db_guard = db.lock().unwrap();
    Json(core_proxy::get_proxy_settings(&db_guard).unwrap())
}

async fn save_proxy_settings(State(db): State<Arc<Mutex<AgentDb>>>, Json(payload): Json<ProxySettings>) {
    let db_guard = db.lock().unwrap();
    core_proxy::save_proxy_settings(&db_guard, payload).unwrap();
}

async fn mcp_get_server_status() -> Json<std::collections::HashMap<String, ServerStatus>> {
    Json(core_mcp::get_server_status().unwrap())
}

#[derive(Deserialize)]
struct McpReadProjectConfigPayload {
    project_path: String,
}

async fn mcp_read_project_config(Json(payload): Json<McpReadProjectConfigPayload>) -> Json<MCPProjectConfig> {
    Json(core_mcp::read_project_config(payload.project_path).unwrap())
}

#[derive(Deserialize)]
struct McpSaveProjectConfigPayload {
    project_path: String,
    config: MCPProjectConfig,
}

async fn mcp_save_project_config(Json(payload): Json<McpSaveProjectConfigPayload>) -> Json<String> {
    Json(core_mcp::save_project_config(payload.project_path, payload.config).unwrap())
}

async fn get_home_directory() -> Json<String> {
    Json(core_claude::get_home_directory().unwrap())
}

async fn list_projects() -> Json<Vec<Project>> {
    Json(core_claude::list_projects().unwrap())
}

#[derive(Deserialize)]
struct CreateProjectPayload {
    path: String,
}

async fn create_project(Json(payload): Json<CreateProjectPayload>) -> Json<Project> {
    Json(core_claude::create_project(payload.path).unwrap())
}

#[derive(Deserialize)]
struct GetProjectSessionsPayload {
    project_id: String,
}

async fn get_project_sessions(Query(params): Query<GetProjectSessionsPayload>) -> Json<Vec<Session>> {
    Json(core_claude::get_project_sessions(params.project_id).unwrap())
}

async fn get_claude_settings() -> Json<ClaudeSettings> {
    Json(core_claude::get_claude_settings().unwrap())
}

async fn get_system_prompt() -> Json<String> {
    Json(core_claude::get_system_prompt().unwrap())
}

#[derive(Deserialize)]
struct SaveSystemPromptPayload {
    content: String,
}

async fn save_system_prompt(Json(payload): Json<SaveSystemPromptPayload>) -> Json<String> {
    Json(core_claude::save_system_prompt(payload.content).unwrap())
}

#[derive(Deserialize)]
struct SaveClaudeSettingsPayload {
    settings: serde_json::Value,
}

async fn save_claude_settings(Json(payload): Json<SaveClaudeSettingsPayload>) -> Json<String> {
    Json(core_claude::save_claude_settings(payload.settings).unwrap())
}

#[derive(Deserialize)]
struct FindClaudeMdFilesPayload {
    project_path: String,
}

async fn find_claude_md_files(Query(params): Query<FindClaudeMdFilesPayload>) -> Json<Vec<ClaudeMdFile>> {
    Json(core_claude::find_claude_md_files(params.project_path).unwrap())
}

#[derive(Deserialize)]
struct ReadClaudeMdFilePayload {
    file_path: String,
}

async fn read_claude_md_file(Query(params): Query<ReadClaudeMdFilePayload>) -> Json<String> {
    Json(core_claude::read_claude_md_file(params.file_path).unwrap())
}

#[derive(Deserialize)]
struct SaveClaudeMdFilePayload {
    file_path: String,
    content: String,
}

async fn save_claude_md_file(Json(payload): Json<SaveClaudeMdFilePayload>) -> Json<String> {
    Json(core_claude::save_claude_md_file(payload.file_path, payload.content).unwrap())
}

#[derive(Deserialize)]
struct LoadSessionHistoryPayload {
    session_id: String,
    project_id: String,
}

async fn load_session_history(Query(params): Query<LoadSessionHistoryPayload>) -> Json<Vec<serde_json::Value>> {
    Json(core_claude::load_session_history(params.session_id, params.project_id).unwrap())
}

#[derive(Deserialize)]
struct ListDirectoryContentsPayload {
    directory_path: String,
}

async fn list_directory_contents(Query(params): Query<ListDirectoryContentsPayload>) -> Json<Vec<FileEntry>> {
    Json(core_claude::list_directory_contents(params.directory_path).unwrap())
}

#[derive(Deserialize)]
struct SearchFilesPayload {
    base_path: String,
    query: String,
}

async fn search_files(Query(params): Query<SearchFilesPayload>) -> Json<Vec<FileEntry>> {
    Json(core_claude::search_files(params.base_path, params.query).unwrap())
}

async fn list_agents(State(db): State<Arc<Mutex<AgentDb>>>) -> Json<Vec<Agent>> {
    let db_guard = db.lock().unwrap();
    Json(core_agents::list_agents(&db_guard).unwrap())
}

#[derive(Deserialize)]
struct CreateAgentPayload {
    name: String,
    icon: String,
    system_prompt: String,
    default_task: Option<String>,
    model: Option<String>,
    enable_file_read: Option<bool>,
    enable_file_write: Option<bool>,
    enable_network: Option<bool>,
    hooks: Option<String>,
}

async fn create_agent(State(db): State<Arc<Mutex<AgentDb>>>, Json(payload): Json<CreateAgentPayload>) -> Json<Agent> {
    let db_guard = db.lock().unwrap();
    Json(core_agents::create_agent(&db_guard, payload.name, payload.icon, payload.system_prompt, payload.default_task, payload.model, payload.enable_file_read, payload.enable_file_write, payload.enable_network, payload.hooks).unwrap())
}

#[derive(Deserialize)]
struct UpdateAgentPayload {
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
}

async fn update_agent(State(db): State<Arc<Mutex<AgentDb>>>, Json(payload): Json<UpdateAgentPayload>) -> Json<Agent> {
    let db_guard = db.lock().unwrap();
    Json(core_agents::update_agent(&db_guard, payload.id, payload.name, payload.icon, payload.system_prompt, payload.default_task, payload.model, payload.enable_file_read, payload.enable_file_write, payload.enable_network, payload.hooks).unwrap())
}

#[derive(Deserialize)]
struct DeleteAgentPayload {
    id: i64,
}

async fn delete_agent(State(db): State<Arc<Mutex<AgentDb>>>, Query(params): Query<DeleteAgentPayload>) {
    let db_guard = db.lock().unwrap();
    core_agents::delete_agent(&db_guard, params.id).unwrap();
}

#[derive(Deserialize)]
struct GetAgentPayload {
    id: i64,
}

async fn get_agent(State(db): State<Arc<Mutex<AgentDb>>>, Query(params): Query<GetAgentPayload>) -> Json<Agent> {
    let db_guard = db.lock().unwrap();
    Json(core_agents::get_agent(&db_guard, params.id).unwrap())
}

#[derive(Deserialize)]
struct ListAgentRunsPayload {
    agent_id: Option<i64>,
}

async fn list_agent_runs(State(db): State<Arc<Mutex<AgentDb>>>, Query(params): Query<ListAgentRunsPayload>) -> Json<Vec<AgentRun>> {
    let db_guard = db.lock().unwrap();
    Json(core_agents::list_agent_runs(&db_guard, params.agent_id).unwrap())
}

#[derive(Deserialize)]
struct GetAgentRunPayload {
    id: i64,
}

async fn get_agent_run(State(db): State<Arc<Mutex<AgentDb>>>, Query(params): Query<GetAgentRunPayload>) -> Json<AgentRun> {
    let db_guard = db.lock().unwrap();
    Json(core_agents::get_agent_run(&db_guard, params.id).unwrap())
}

#[derive(Deserialize)]
struct GetAgentRunWithRealTimeMetricsPayload {
    id: i64,
}

async fn get_agent_run_with_real_time_metrics(State(db): State<Arc<Mutex<AgentDb>>>, Query(params): Query<GetAgentRunWithRealTimeMetricsPayload>) -> Json<AgentRunWithMetrics> {
    let db_guard = db.lock().unwrap();
    Json(core_agents::get_agent_run_with_real_time_metrics(&db_guard, params.id).await.unwrap())
}

#[derive(Deserialize)]
struct ListAgentRunsWithMetricsPayload {
    agent_id: Option<i64>,
}

async fn list_agent_runs_with_metrics(State(db): State<Arc<Mutex<AgentDb>>>, Query(params): Query<ListAgentRunsWithMetricsPayload>) -> Json<Vec<AgentRunWithMetrics>> {
    let db_guard = db.lock().unwrap();
    Json(core_agents::list_agent_runs_with_metrics(&db_guard, params.agent_id).await.unwrap())
}

#[derive(Deserialize)]
struct ExportAgentPayload {
    id: i64,
}

async fn export_agent(State(db): State<Arc<Mutex<AgentDb>>>, Query(params): Query<ExportAgentPayload>) -> Json<String> {
    let db_guard = db.lock().unwrap();
    Json(core_agents::export_agent(&db_guard, params.id).unwrap())
}

#[derive(Deserialize)]
struct ImportAgentPayload {
    json_data: String,
}

async fn import_agent(State(db): State<Arc<Mutex<AgentDb>>>, Json(payload): Json<ImportAgentPayload>) -> Json<Agent> {
    let db_guard = db.lock().unwrap();
    Json(core_agents::import_agent(&db_guard, payload.json_data).unwrap())
}

async fn fetch_github_agents() -> Json<Vec<GitHubAgentFile>> {
    Json(core_agents::fetch_github_agents().await.unwrap())
}

#[derive(Deserialize)]
struct FetchGitHubAgentContentPayload {
    download_url: String,
}

async fn fetch_github_agent_content(Query(params): Query<FetchGitHubAgentContentPayload>) -> Json<AgentExport> {
    Json(core_agents::fetch_github_agent_content(params.download_url).await.unwrap())
}

#[derive(Deserialize)]
struct ImportAgentFromGitHubPayload {
    download_url: String,
}

async fn import_agent_from_github(State(db): State<Arc<Mutex<AgentDb>>>, Json(payload): Json<ImportAgentFromGitHubPayload>) -> Json<Agent> {
    let db_guard = db.lock().unwrap();
    Json(core_agents::import_agent_from_github(&db_guard, payload.download_url).await.unwrap())
}

#[tokio::main]
pub async fn main() {
    let app_dir = dirs::home_dir().unwrap().join(".claude");
    std::fs::create_dir_all(&app_dir).unwrap();
    let db_path = app_dir.join("agents.db");
    let conn = init_database_web(&db_path).unwrap();
    let db = Arc::new(Mutex::new(AgentDb(Mutex::new(conn))));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/usage/stats", get(get_usage_stats))
        .route("/usage/by_date_range", get(get_usage_by_date_range))
        .route("/usage/details", get(get_usage_details))
        .route("/usage/session_stats", get(get_session_stats))
        .route("/storage/tables", get(storage_list_tables))
        .route("/storage/table", get(storage_read_table))
        .route("/storage/update", post(storage_update_row))
        .route("/storage/delete", post(storage_delete_row))
        .route("/storage/insert", post(storage_insert_row))
        .route("/storage/execute", post(storage_execute_sql))
        .route("/slash_commands/list", get(slash_commands_list))
        .route("/slash_commands/get", get(slash_command_get))
        .route("/slash_commands/save", post(slash_command_save))
        .route("/slash_commands/delete", post(slash_command_delete))
        .route("/proxy/settings", get(get_proxy_settings))
        .route("/proxy/settings", post(save_proxy_settings))
        .route("/mcp/status", get(mcp_get_server_status))
        .route("/mcp/project_config", get(mcp_read_project_config))
        .route("/mcp/project_config", post(mcp_save_project_config))
        .route("/claude/home_directory", get(get_home_directory))
        .route("/claude/projects", get(list_projects))
        .route("/claude/projects", post(create_project))
        .route("/claude/sessions", get(get_project_sessions))
        .route("/claude/settings", get(get_claude_settings))
        .route("/claude/system_prompt", get(get_system_prompt))
        .route("/claude/system_prompt", post(save_system_prompt))
        .route("/claude/settings", post(save_claude_settings))
        .route("/claude/md_files", get(find_claude_md_files))
        .route("/claude/md_file", get(read_claude_md_file))
        .route("/claude/md_file", post(save_claude_md_file))
        .route("/claude/session_history", get(load_session_history))
        .route("/claude/directory", get(list_directory_contents))
        .route("/claude/search", get(search_files))
        .route("/agents/list", get(list_agents))
        .route("/agents/create", post(create_agent))
        .route("/agents/update", post(update_agent))
        .route("/agents/delete", get(delete_agent))
        .route("/agents/get", get(get_agent))
        .route("/agents/runs", get(list_agent_runs))
        .route("/agents/run", get(get_agent_run))
        .route("/agents/run_with_metrics", get(get_agent_run_with_real_time_metrics))
        .route("/agents/runs_with_metrics", get(list_agent_runs_with_metrics))
        .route("/agents/export", get(export_agent))
        .route("/agents/import", post(import_agent))
        .route("/agents/github", get(fetch_github_agents))
        .route("/agents/github/content", get(fetch_github_agent_content))
        .route("/agents/github/import", post(import_agent_from_github))
        .with_state(db)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}