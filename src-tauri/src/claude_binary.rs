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
    use std::process::Command;

    let mut cmd = Command::new(program);

    // Propagate common env vars (incl. proxies)
    for (key, value) in std::env::vars() {
        if key == "PATH"
            || key == "HOME"
            || key == "USER"
            || key == "SHELL"
            || key == "LANG"
            || key == "LC_ALL"
            || key.starts_with("LC_")
            || key == "NODE_PATH"
            || key == "NVM_DIR"
            || key == "NVM_BIN"
            || key == "HOMEBREW_PREFIX"
            || key == "HOMEBREW_CELLAR"
            || key == "HTTP_PROXY"
            || key == "HTTPS_PROXY"
            || key == "NO_PROXY"
            || key == "ALL_PROXY"
        {
            cmd.env(&key, &value);
        }
    }

    // If using an NVM-installed binary, prepend its bin directory to PATH
    if program.contains("/.nvm/versions/node/") {
        if let Some(node_bin_dir) = std::path::Path::new(program).parent() {
            let current_path = std::env::var("PATH").unwrap_or_default();
            let node_bin_str = node_bin_dir.to_string_lossy();
            if !current_path.contains(&node_bin_str.as_ref()) {
                let new_path = format!("{}:{}", node_bin_str, current_path);
                cmd.env("PATH", new_path);
            }
        }
    }

    // If using a Homebrew-installed binary, ensure its bin is in PATH
    if program.contains("/homebrew/") || program.contains("/opt/homebrew/") {
        if let Some(program_dir) = std::path::Path::new(program).parent() {
            let current_path = std::env::var("PATH").unwrap_or_default();
            let brew_bin_str = program_dir.to_string_lossy();
            if !current_path.contains(&brew_bin_str.as_ref()) {
                let new_path = format!("{}:{}", brew_bin_str, current_path);
                cmd.env("PATH", new_path);
            }
        }
    }

    cmd
}
