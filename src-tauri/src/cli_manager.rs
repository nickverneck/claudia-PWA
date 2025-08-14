// src-tauri/src/cli_manager.rs

use tauri::{AppHandle, Manager, Emitter};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::process::Stdio;
use log::{info, error, warn};

// Import executor modules to avoid symbol conflicts
use crate::cli_executors::{
    claude as claude_exec,
    gemini as gemini_exec,
    codex as openai_exec,
    qwen as qwen_exec,
    aider as aider_exec,
};
use crate::process::ProcessRegistryState;

// Define a generic CLI provider enum
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CliProvider {
    Claude,
    Gemini,
    OpenAI,
    Qwen,
    Aider,
}

// Define a struct to hold information about a running CLI process
#[derive(Debug, Clone)]
pub struct CliProcess {
    pub id: u32, // Process ID
    pub provider: CliProvider,
    pub model: String,
    pub project_path: String,
    pub task: String,
    pub session_id: String, // This will be the run_id from the frontend
    // Add other relevant fields as needed
}

// Global state to keep track of running processes
pub type RunningProcesses = Arc<Mutex<HashMap<u32, CliProcess>>>;

/// Helper to build a tokio Command with correct environment propagation
fn create_tokio_command_with_env(program: &str) -> Command {
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

    cmd
}

// Function to execute a CLI command
pub async fn execute_cli_command(
    app_handle: AppHandle,
    provider: CliProvider,
    model: String,
    project_path: String,
    task: String,
    session_id: String, // This is the run_id from the frontend
    // Add other parameters as needed for specific CLIs
) -> Result<u32, String> {
    let mut command = match provider {
        CliProvider::Claude => {
            info!("Executing Claude command with model: {}", model);
            let installations = claude_exec::discover_claude_installations();
            let claude_path = if let Some(best) = claude_exec::select_best_installation(installations) {
                best.path
            } else {
                error!("No valid Claude installation found.");
                return Err("No valid Claude installation found.".to_string());
            };
            let mut cmd_builder = create_tokio_command_with_env(&claude_path);
            cmd_builder.args(&["code", "--model", &model, "--project", &project_path, "--task", &task]);
            cmd_builder
        },
        CliProvider::Gemini => {
            info!("Executing Gemini command with model: {}", model);
            let installations = gemini_exec::discover_gemini_installations();
            let gemini_path = if let Some(best) = gemini_exec::select_best_installation(installations) {
                best.path
            } else {
                error!("No valid Gemini installation found.");
                return Err("No valid Gemini installation found.".to_string());
            };
            let mut cmd_builder = create_tokio_command_with_env(&gemini_path);
            cmd_builder.args(&["cli", "run", "--model", &model, "--project", &project_path, "--task", &task]);
            cmd_builder
        },
        CliProvider::OpenAI => {
            info!("Executing Codex command with model: {}", model);
            let installations = openai_exec::discover_codex_installations();
            let codex_path = if let Some(best) = openai_exec::select_best_installation(installations) {
                best.path
            } else {
                error!("No valid Codex installation found.");
                return Err("No valid Codex installation found.".to_string());
            };
            let mut cmd_builder = create_tokio_command_with_env(&codex_path);
            cmd_builder.args(&["run", "--model", &model, "--project", &project_path, "--task", &task]);
            cmd_builder
        },
        CliProvider::Qwen => {
            info!("Executing Qwen command with model: {}", model);
            let installations = qwen_exec::discover_qwen_installations();
            let qwen_path = if let Some(best) = qwen_exec::select_best_installation(installations) {
                best.path
            } else {
                error!("No valid Qwen installation found.");
                return Err("No valid Qwen installation found.".to_string());
            };
            let mut cmd_builder = create_tokio_command_with_env(&qwen_path);
            cmd_builder.args(&["code", "--model", &model, "--project", &project_path, "--task", &task]);
            cmd_builder
        },
        CliProvider::Aider => {
            info!("Executing Aider command with model: {}", model);
            let installations = aider_exec::discover_aider_installations();
            let aider_path = if let Some(best) = aider_exec::select_best_installation(installations) {
                best.path
            } else {
                error!("No valid Aider installation found.");
                return Err("No valid Aider installation found.".to_string());
            };
            let mut cmd_builder = create_tokio_command_with_env(&aider_path);
            cmd_builder.args(&["--model", &model, "--project", &project_path, "--task", &task]);
            cmd_builder
        },
    };

    command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child: Child = command
        .spawn()
        .map_err(|e| format!("Failed to spawn command: {}", e))?;

    let pid = child.id().ok_or("Failed to get process ID".to_string())?;

    // Register this Claude session in the global ProcessRegistry so cancellation/lookups work
    let run_id_opt: Option<i64> = {
        let registry = app_handle.state::<ProcessRegistryState>();
        match registry.0.register_claude_session(
            session_id.clone(),
            pid,
            project_path.clone(),
            task.clone(),
            model.clone(),
        ) {
            Ok(id) => Some(id),
            Err(e) => {
                warn!("Failed to register Claude session in ProcessRegistry: {}", e);
                None
            }
        }
    };

    // Store the running process (clone Arc out of State to avoid lifetime issues)
    let processes_arc: RunningProcesses = {
        let state = app_handle.state::<RunningProcesses>();
        state.inner().clone()
    };
    processes_arc.lock().unwrap().insert(pid, CliProcess {
        id: pid,
        provider: provider.clone(),
        model: model.clone(),
        project_path: project_path.clone(),
        task: task.clone(),
        session_id: session_id.clone(),
    });

    // Spawn a task to read stdout and emit events
    let stdout = child.stdout.take().ok_or("Failed to get stdout".to_string())?;
    let app_handle_clone = app_handle.clone();
    let session_id_clone = session_id.clone();
    tokio::spawn(async move {
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await.unwrap_or(None) {
            // Emit a generic event for agent output
            // Also append to ProcessRegistry live output if available
            if let Some(run_id) = run_id_opt {
                let registry = app_handle_clone.state::<ProcessRegistryState>();
                let _ = registry.0.append_live_output(run_id, &format!("{}\n", line));
            }
            app_handle_clone.emit(&format!("agent-output:{}", session_id_clone), line).unwrap();
        }
    });

    // Spawn a task to read stderr and emit events
    let stderr = child.stderr.take().ok_or("Failed to get stderr".to_string())?;
    let app_handle_clone = app_handle.clone();
    let session_id_clone = session_id.clone();
    tokio::spawn(async move {
        let reader = BufReader::new(stderr);
        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await.unwrap_or(None) {
            // Emit a generic event for agent error
            if let Some(run_id) = run_id_opt {
                let registry = app_handle_clone.state::<ProcessRegistryState>();
                let _ = registry.0.append_live_output(run_id, &format!("{}\n", line));
            }
            app_handle_clone.emit(&format!("agent-error:{}", session_id_clone), line).unwrap();
        }
    });

    // Spawn a task to wait for the process to finish
    let app_handle_clone = app_handle.clone();
    let session_id_clone = session_id.clone();
    let running_processes_clone = processes_arc.clone();
    tokio::spawn(async move {
        let status = child.wait().await.unwrap();
        let success = status.success();
        // Emit a generic event for agent completion
        app_handle_clone
            .emit(&format!("agent-complete:{}", session_id_clone), success)
            .unwrap();

        // Unregister from ProcessRegistry if we registered earlier
        if let Some(run_id) = run_id_opt {
            let registry = app_handle_clone.state::<ProcessRegistryState>();
            let _ = registry.0.unregister_process(run_id);
        }

        // Remove from running processes
        running_processes_clone.lock().unwrap().remove(&pid);
    });

    Ok(pid)
}

// Function to kill a running CLI process
pub async fn kill_cli_process(app_handle: AppHandle, pid: u32) -> Result<bool, String> {
    let processes_arc: RunningProcesses = {
        let state = app_handle.state::<RunningProcesses>();
        state.inner().clone()
    };
    // Remove from map first to avoid holding the lock across await
    let should_kill = {
        let mut processes = processes_arc.lock().unwrap();
        processes.remove(&pid).is_some()
    };

    if should_kill {
        // Attempt to kill the process (no lock held)
        #[cfg(target_os = "windows")]
        {
            Command::new("taskkill")
                .args(&["/F", "/PID", &pid.to_string()])
                .status()
                .await
                .map_err(|e| format!("Failed to kill process on Windows: {}", e))?;
        }
        #[cfg(not(target_os = "windows"))]
        {
            Command::new("kill")
                .args(&["-9", &pid.to_string()])
                .status()
                .await
                .map_err(|e| format!("Failed to kill process: {}", e))?;
        }
        Ok(true)
    } else {
        Err(format!("Process with PID {} not found.", pid))
    }
}

// Function to list running CLI processes
pub async fn list_running_cli_processes(app_handle: AppHandle) -> Result<Vec<CliProcess>, String> {
    let running_processes = app_handle.state::<RunningProcesses>();
    let processes: Vec<CliProcess> = running_processes.lock().unwrap().values().cloned().collect();
    Ok(processes)
}
