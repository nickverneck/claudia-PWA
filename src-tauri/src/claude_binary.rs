// src-tauri/src/claude_binary.rs
// Compatibility wrapper that re-exports functionality from cli_executors::claude

pub use crate::cli_executors::claude::{
    ClaudeInstallation,
};

/// Find the Claude binary using the executor's discovery logic
pub fn find_claude_binary(app_handle: &tauri::AppHandle) -> Result<String, String> {
    crate::cli_executors::claude::find_claude_binary(app_handle)
}

/// Discover available Claude installations
pub fn discover_claude_installations() -> Vec<ClaudeInstallation> {
    crate::cli_executors::claude::discover_claude_installations()
}

/// Select the best installation among discovered ones
pub fn select_best_installation(installations: Vec<ClaudeInstallation>) -> Option<ClaudeInstallation> {
    crate::cli_executors::claude::select_best_installation(installations)
}

/// Create a std::process::Command with properly propagated environment
pub fn create_command_with_env(program: &str) -> std::process::Command {
    crate::cli_executors::claude::create_command_with_env(program)
}
