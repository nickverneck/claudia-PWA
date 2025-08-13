// src-tauri/src/cli_manager.rs

use tauri::{AppHandle, Manager};
use tokio::process::Command;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::cli_executors::claude::{discover_claude_installations, select_best_installation, create_command_with_env};
use crate::cli_executors::gemini::{discover_gemini_installations, select_best_installation, create_command_with_env};
use crate::cli_executors::openai_codex::{discover_openai_codex_installations, select_best_installation, create_command_with_env};
use crate::cli_executors::qwen::{discover_qwen_installations, select_best_installation, create_command_with_env};
use crate::cli_executors::aider::{discover_aider_installations, select_best_installation, create_command_with_env};
use log::{info, error};

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
            let installations = discover_claude_installations();
            let claude_path = if let Some(best) = select_best_installation(installations) {
                best.path
            } else {
                error!("No valid Claude installation found.");
                return Err("No valid Claude installation found.".to_string());
            };
            let mut cmd_builder = create_command_with_env(&claude_path);
            cmd_builder.args(&["code", "--model", &model, "--project", &project_path, "--task", &task]);
            cmd_builder
        },
        CliProvider::Gemini => {
            info!("Executing Gemini command with model: {}", model);
            let installations = discover_gemini_installations();
            let gemini_path = if let Some(best) = select_best_installation(installations) {
                best.path
            } else {
                error!("No valid Gemini installation found.");
                return Err("No valid Gemini installation found.".to_string());
            };
            let mut cmd_builder = create_command_with_env(&gemini_path);
            cmd_builder.args(&["cli", "run", "--model", &model, "--project", &project_path, "--task", &task]);
            cmd_builder
        },
        CliProvider::OpenAI => {
            info!("Executing OpenAI Codex command with model: {}", model);
            let installations = discover_openai_codex_installations();
            let openai_codex_path = if let Some(best) = select_best_installation(installations) {
                best.path
            } else {
                error!("No valid OpenAI Codex installation found.");
                return Err("No valid OpenAI Codex installation found.".to_string());
            };
            let mut cmd_builder = create_command_with_env(&openai_codex_path);
            cmd_builder.args(&["run", "--model", &model, "--project", &project_path, "--task", &task]);
            cmd_builder
        },
        CliProvider::Qwen => {
            info!("Executing Qwen command with model: {}", model);
            let installations = discover_qwen_installations();
            let qwen_path = if let Some(best) = select_best_installation(installations) {
                best.path
            } else {
                error!("No valid Qwen installation found.");
                return Err("No valid Qwen installation found.".to_string());
            };
            let mut cmd_builder = create_command_with_env(&qwen_path);
            cmd_builder.args(&["code", "--model", &model, "--project", &project_path, "--task", &task]);
            cmd_builder
        },
        CliProvider::Aider => {
            info!("Executing Aider command with model: {}", model);
            let installations = discover_aider_installations();
            let aider_path = if let Some(best) = select_best_installation(installations) {
                best.path
            } else {
                error!("No valid Aider installation found.");
                return Err("No valid Aider installation found.".to_string());
            };
            let mut cmd_builder = create_command_with_env(&aider_path);
            cmd_builder.args(&["--model", &model, "--project", &project_path, "--task", &task]);
            cmd_builder
        },
    };

    command
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn command: {}", e))?;

    let pid = command.id().ok_or("Failed to get process ID".to_string())?;

    // Store the running process
    let running_processes = app_handle.state::<RunningProcesses>();
    running_processes.lock().unwrap().insert(pid, CliProcess {
        id: pid,
        provider: provider.clone(),
        model: model.clone(),
        project_path: project_path.clone(),
        task: task.clone(),
        session_id: session_id.clone(),
    });

    // Spawn a task to read stdout and emit events
    let stdout = command.stdout.take().ok_or("Failed to get stdout".to_string())?;
    let app_handle_clone = app_handle.clone();
    let session_id_clone = session_id.clone();
    tokio::spawn(async move {
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await.unwrap_or(None) {
            // Emit a generic event for agent output
            app_handle_clone.emit_all(&format!("agent-output:{}", session_id_clone), line).unwrap();
        }
    });

    // Spawn a task to read stderr and emit events
    let stderr = command.stderr.take().ok_or("Failed to get stderr".to_string())?;
    let app_handle_clone = app_handle.clone();
    let session_id_clone = session_id.clone();
    tokio::spawn(async move {
        let reader = BufReader::new(stderr);
        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await.unwrap_or(None) {
            // Emit a generic event for agent error
            app_handle_clone.emit_all(&format!("agent-error:{}", session_id_clone), line).unwrap();
        }
    });

    // Spawn a task to wait for the process to finish
    let app_handle_clone = app_handle.clone();
    let session_id_clone = session_id.clone();
    let running_processes_clone = running_processes.clone();
    tokio::spawn(async move {
        let status = command.wait().await.unwrap();
        let success = status.success();
        // Emit a generic event for agent completion
        app_handle_clone.emit_all(&format!("agent-complete:{}", session_id_clone), success).unwrap();

        // Remove from running processes
        running_processes_clone.lock().unwrap().remove(&pid);
    });

    Ok(pid)
}

// Function to kill a running CLI process
pub async fn kill_cli_process(app_handle: AppHandle, pid: u32) -> Result<bool, String> {
    let running_processes = app_handle.state::<RunningProcesses>();
    let mut processes = running_processes.lock().unwrap();

    if let Some(process_info) = processes.remove(&pid) {
        // Attempt to kill the process
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
