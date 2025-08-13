use crate::core_logic::claude::{self as core_claude, ClaudeSettings, ClaudeVersionStatus, ClaudeMdFile, FileEntry, Project, Session};
use tauri::{command, AppHandle, State};

#[command]
pub async fn get_home_directory() -> Result<String, String> {
    core_claude::get_home_directory()
}

#[command]
pub async fn list_projects() -> Result<Vec<Project>, String> {
    core_claude::list_projects()
}

#[command]
pub async fn create_project(path: String) -> Result<Project, String> {
    core_claude::create_project(path)
}

#[command]
pub async fn get_project_sessions(project_id: String) -> Result<Vec<Session>, String> {
    core_claude::get_project_sessions(project_id)
}

#[command]
pub async fn get_claude_settings() -> Result<ClaudeSettings, String> {
    core_claude::get_claude_settings()
}

#[command]
pub async fn open_new_session(app: AppHandle, path: Option<String>) -> Result<String, String> {
    core_claude::open_new_session(&app, path)
}

#[command]
pub async fn get_system_prompt() -> Result<String, String> {
    core_claude::get_system_prompt()
}

#[command]
pub async fn check_claude_version(app: AppHandle) -> Result<ClaudeVersionStatus, String> {
    core_claude::check_claude_version(&app)
}

#[command]
pub async fn save_system_prompt(content: String) -> Result<String, String> {
    core_claude::save_system_prompt(content)
}

#[command]
pub async fn save_claude_settings(settings: serde_json::Value) -> Result<String, String> {
    core_claude::save_claude_settings(settings)
}

#[command]
pub async fn find_claude_md_files(project_path: String) -> Result<Vec<ClaudeMdFile>, String> {
    core_claude::find_claude_md_files(project_path)
}

#[command]
pub async fn read_claude_md_file(file_path: String) -> Result<String, String> {
    core_claude::read_claude_md_file(file_path)
}

#[command]
pub async fn save_claude_md_file(file_path: String, content: String) -> Result<String, String> {
    core_claude::save_claude_md_file(file_path, content)
}

#[command]
pub async fn load_session_history(
    session_id: String,
    project_id: String,
) -> Result<Vec<serde_json::Value>, String> {
    core_claude::load_session_history(session_id, project_id)
}

#[command]
pub async fn execute_claude_code(
    app: AppHandle,
    project_path: String,
    prompt: String,
    model: String,
) -> Result<(), String> {
    core_claude::execute_claude_code(app, project_path, prompt, model).await
}

#[command]
pub async fn continue_claude_code(
    app: AppHandle,
    project_path: String,
    prompt: String,
    model: String,
) -> Result<(), String> {
    core_claude::continue_claude_code(app, project_path, prompt, model).await
}

#[command]
pub async fn resume_claude_code(
    app: AppHandle,
    project_path: String,
    session_id: String,
    prompt: String,
    model: String,
) -> Result<(), String> {
    core_claude::resume_claude_code(app, project_path, session_id, prompt, model).await
}

#[command]
pub async fn cancel_claude_execution(
    app: AppHandle,
    session_id: Option<String>,
) -> Result<(), String> {
    core_claude::cancel_claude_execution(app, session_id).await
}

#[command]
pub async fn list_running_claude_sessions(
    registry: State<'_, crate::process::ProcessRegistryState>,
) -> Result<Vec<crate::process::ProcessInfo>, String> {
    core_claude::list_running_claude_sessions(registry)
}

#[command]
pub async fn get_claude_session_output(
    registry: State<'_, crate::process::ProcessRegistryState>,
    session_id: String,
) -> Result<String, String> {
    core_claude::get_claude_session_output(registry, session_id)
}

#[command]
pub async fn list_directory_contents(directory_path: String) -> Result<Vec<FileEntry>, String> {
    core_claude::list_directory_contents(directory_path)
}

#[command]
pub async fn search_files(base_path: String, query: String) -> Result<Vec<FileEntry>, String> {
    core_claude::search_files(base_path, query)
}

#[command]
pub async fn create_checkpoint(
    app: State<'_, crate::checkpoint::state::CheckpointState>,
    session_id: String,
    project_id: String,
    project_path: String,
    message_index: Option<usize>,
    description: Option<String>,
) -> Result<crate::checkpoint::CheckpointResult, String> {
    core_claude::create_checkpoint(app, session_id, project_id, project_path, message_index, description).await
}

#[command]
pub async fn restore_checkpoint(
    app: State<'_, crate::checkpoint::state::CheckpointState>,
    checkpoint_id: String,
    session_id: String,
    project_id: String,
    project_path: String,
) -> Result<crate::checkpoint::CheckpointResult, String> {
    core_claude::restore_checkpoint(app, checkpoint_id, session_id, project_id, project_path).await
}

#[command]
pub async fn list_checkpoints(
    app: State<'_, crate::checkpoint::state::CheckpointState>,
    session_id: String,
    project_id: String,
    project_path: String,
) -> Result<Vec<crate::checkpoint::Checkpoint>, String> {
    core_claude::list_checkpoints(app, session_id, project_id, project_path).await
}

#[command]
pub async fn fork_from_checkpoint(
    app: State<'_, crate::checkpoint::state::CheckpointState>,
    checkpoint_id: String,
    session_id: String,
    project_id: String,
    project_path: String,
    new_session_id: String,
    description: Option<String>,
) -> Result<crate::checkpoint::CheckpointResult, String> {
    core_claude::fork_from_checkpoint(app, checkpoint_id, session_id, project_id, project_path, new_session_id, description).await
}

#[command]
pub async fn get_session_timeline(
    app: State<'_, crate::checkpoint::state::CheckpointState>,
    session_id: String,
    project_id: String,
    project_path: String,
) -> Result<crate::checkpoint::SessionTimeline, String> {
    core_claude::get_session_timeline(app, session_id, project_id, project_path).await
}

#[command]
pub async fn update_checkpoint_settings(
    app: State<'_, crate::checkpoint::state::CheckpointState>,
    session_id: String,
    project_id: String,
    project_path: String,
    auto_checkpoint_enabled: bool,
    checkpoint_strategy: String,
) -> Result<(), String> {
    core_claude::update_checkpoint_settings(app, session_id, project_id, project_path, auto_checkpoint_enabled, checkpoint_strategy).await
}

#[command]
pub async fn get_checkpoint_diff(
    from_checkpoint_id: String,
    to_checkpoint_id: String,
    session_id: String,
    project_id: String,
) -> Result<crate::checkpoint::CheckpointDiff, String> {
    core_claude::get_checkpoint_diff(from_checkpoint_id, to_checkpoint_id, session_id, project_id).await
}

#[command]
pub async fn track_checkpoint_message(
    app: State<'_, crate::checkpoint::state::CheckpointState>,
    session_id: String,
    project_id: String,
    project_path: String,
    message: String,
) -> Result<(), String> {
    core_claude::track_checkpoint_message(app, session_id, project_id, project_path, message).await
}

#[command]
pub async fn check_auto_checkpoint(
    app: State<'_, crate::checkpoint::state::CheckpointState>,
    session_id: String,
    project_id: String,
    project_path: String,
    message: String,
) -> Result<bool, String> {
    core_claude::check_auto_checkpoint(app, session_id, project_id, project_path, message).await
}

#[command]
pub async fn cleanup_old_checkpoints(
    app: State<'_, crate::checkpoint::state::CheckpointState>,
    session_id: String,
    project_id: String,
    project_path: String,
    keep_count: usize,
) -> Result<usize, String> {
    core_claude::cleanup_old_checkpoints(app, session_id, project_id, project_path, keep_count).await
}

#[command]
pub async fn get_checkpoint_settings(
    app: State<'_, crate::checkpoint::state::CheckpointState>,
    session_id: String,
    project_id: String,
    project_path: String,
) -> Result<serde_json::Value, String> {
    core_claude::get_checkpoint_settings(app, session_id, project_id, project_path).await
}

#[command]
pub async fn clear_checkpoint_manager(
    app: State<'_, crate::checkpoint::state::CheckpointState>,
    session_id: String,
) -> Result<(), String> {
    core_claude::clear_checkpoint_manager(app, session_id).await
}

#[command]
pub async fn get_checkpoint_state_stats(
    app: State<'_, crate::checkpoint::state::CheckpointState>,
) -> Result<serde_json::Value, String> {
    core_claude::get_checkpoint_state_stats(app).await
}

#[command]
pub async fn get_recently_modified_files(
    app: State<'_, crate::checkpoint::state::CheckpointState>,
    session_id: String,
    project_id: String,
    project_path: String,
    minutes: i64,
) -> Result<Vec<String>, String> {
    core_claude::get_recently_modified_files(app, session_id, project_id, project_path, minutes).await
}
