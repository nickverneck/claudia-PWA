// src-tauri/src/cli_executors/openai_codex.rs

use anyhow::Result;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::path::PathBuf;
use std::process::Command;
use tauri::Manager;

/// Type of OpenAI Codex installation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InstallationType {
    /// System-installed binary
    System,
    /// Custom path specified by user
    Custom,
}

/// Represents an OpenAI Codex installation with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAICodexInstallation {
    /// Full path to the OpenAI Codex binary
    pub path: String,
    /// Version string if available
    pub version: Option<String>,
    /// Source of discovery (e.g., "system", "homebrew", "which")
    pub source: String,
    /// Type of installation
    pub installation_type: InstallationType,
}

/// Main function to find the OpenAI Codex binary
pub fn find_openai_codex_binary(app_handle: &tauri::AppHandle) -> Result<String, String> {
    info!("Searching for openai_codex binary...");

    // First check if we have a stored path and preference in the database
    if let Ok(app_data_dir) = app_handle.path().app_data_dir() {
        let db_path = app_data_dir.join("agents.db");
        if db_path.exists() {
            if let Ok(conn) = rusqlite::Connection::open(&db_path) {
                // Check for stored path first
                if let Ok(stored_path) = conn.query_row(
                    "SELECT value FROM app_settings WHERE key = 'openai_codex_binary_path'",
                    [],
                    |row| row.get::<_, String>(0),
                ) {
                    info!("Found stored openai_codex path in database: {}", stored_path);
                    
                    // Check if the path still exists
                    let path_buf = PathBuf::from(&stored_path);
                    if path_buf.exists() && path_buf.is_file() {
                        return Ok(stored_path);
                    } else {
                        warn!("Stored openai_codex path no longer exists: {}", stored_path);
                    }
                }
                
                // Check user preference
                let preference = conn.query_row(
                    "SELECT value FROM app_settings WHERE key = 'openai_codex_installation_preference'",
                    [],
                    |row| row.get::<_, String>(0),
                ).unwrap_or_else(|_| "system".to_string());
                
                info!("User preference for OpenAI Codex installation: {}", preference);
            }
        }
    }

    // Discover all available system installations
    let installations = discover_system_installations();

    if installations.is_empty() {
        error!("Could not find openai_codex binary in any location");
        return Err("OpenAI Codex CLI not found. Please ensure it's installed and in your PATH.".to_string());
    }

    // Log all found installations
    for installation in &installations {
        info!("Found OpenAI Codex installation: {:?}", installation);
    }

    // Select the best installation (highest version)
    if let Some(best) = select_best_installation(installations) {
        info!(
            "Selected OpenAI Codex installation: path={}, version={:?}, source={}",
            best.path, best.version, best.source
        );
        Ok(best.path)
    } else {
        Err("No valid OpenAI Codex installation found".to_string())
    }
}

/// Discovers all available OpenAI Codex installations and returns them for selection
pub fn discover_openai_codex_installations() -> Vec<OpenAICodexInstallation> {
    info!("Discovering all OpenAI Codex installations...");

    let mut installations = discover_system_installations();

    // Sort by version (highest first), then by source preference
    installations.sort_by(|a, b| {
        match (&a.version, &b.version) {
            (Some(v1), Some(v2)) => {
                // Compare versions in descending order (newest first)
                match compare_versions(v2, v1) {
                    Ordering::Equal => {
                        // If versions are equal, prefer by source
                        source_preference(a).cmp(&source_preference(b))
                    }
                    other => other,
                }
            }
            (Some(_), None) => Ordering::Less, // Version comes before no version
            (None, Some(_)) => Ordering::Greater,
            (None, None) => source_preference(a).cmp(&source_preference(b)),
        }
    });

    installations
}

/// Returns a preference score for installation sources (lower is better)
fn source_preference(installation: &OpenAICodexInstallation) -> u8 {
    match installation.source.as_str() {
        "which" => 1,
        "homebrew" => 2,
        "system" => 3,
        "local-bin" => 4,
        "PATH" => 5,
        _ => 6,
    }
}

/// Discovers all OpenAI Codex installations on the system
fn discover_system_installations() -> Vec<OpenAICodexInstallation> {
    let mut installations = Vec::new();

    // 1. Try 'which' command first
    if let Some(installation) = try_which_command() {
        installations.push(installation);
    }

    // 2. Check standard paths
    installations.extend(find_standard_installations());

    // Remove duplicates by path
    let mut unique_paths = std::collections::HashSet::new();
    installations.retain(|install| unique_paths.insert(install.path.clone()));

    installations
}

/// Try using the 'which' command to find OpenAI Codex
fn try_which_command() -> Option<OpenAICodexInstallation> {
    debug!("Trying 'which openai_codex' to find binary...");

    match Command::new("which").arg("openai_codex").output() {
        Ok(output) if output.status.success() => {
            let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

            if output_str.is_empty() {
                return None;
            }

            let path = if output_str.starts_with("openai_codex:") && output_str.contains("aliased to") {
                output_str
                    .split("aliased to")
                    .nth(1)
                    .map(|s| s.trim().to_string())
            } else {
                Some(output_str)
            }?;

            debug!("'which' found openai_codex at: {}", path);

            // Verify the path exists
            if !PathBuf::from(&path).exists() {
                warn!("Path from 'which' does not exist: {}", path);
                return None;
            }

            // Get version
            let version = get_openai_codex_version(&path).ok().flatten();

            Some(OpenAICodexInstallation {
                path,
                version,
                source: "which".to_string(),
                installation_type: InstallationType::System,
            })
        }
        _ => None,
    }
}

/// Check standard installation paths
fn find_standard_installations() -> Vec<OpenAICodexInstallation> {
    let mut installations = Vec::new();

    // Common installation paths for openai_codex
    let mut paths_to_check: Vec<(String, String)> = vec![
        ("/usr/local/bin/openai_codex".to_string(), "system".to_string()),
        (
            "/opt/homebrew/bin/openai_codex".to_string(),
            "homebrew".to_string(),
        ),
        ("/usr/bin/openai_codex".to_string(), "system".to_string()),
        ("/bin/openai_codex".to_string(), "system".to_string()),
    ];

    // Also check user-specific paths
    if let Ok(home) = std::env::var("HOME") {
        paths_to_check.extend(vec![
            (
                format!("{}/.local/bin/openai_codex", home),
                "local-bin".to_string(),
            ),
        ]);
    }

    // Check each path
    for (path, source) in paths_to_check {
        let path_buf = PathBuf::from(&path);
        if path_buf.exists() && path_buf.is_file() {
            debug!("Found openai_codex at standard path: {} ({})", path, source);

            // Get version
            let version = get_openai_codex_version(&path).ok().flatten();

            installations.push(OpenAICodexInstallation {
                path,
                version,
                source,
                installation_type: InstallationType::System,
            });
        }
    }

    // Also check if openai_codex is available in PATH (without full path)
    if let Ok(output) = Command::new("openai_codex").arg("--version").output() {
        if output.status.success() {
            debug!("openai_codex is available in PATH");
            let version = extract_version_from_output(&output.stdout);

            installations.push(OpenAICodexInstallation {
                path: "openai_codex".to_string(),
                version,
                source: "PATH".to_string(),
                installation_type: InstallationType::System,
            });
        }
    }

    installations
}

/// Get OpenAI Codex version by running --version command
fn get_openai_codex_version(path: &str) -> Result<Option<String>, String> {
    match Command::new(path).arg("--version").output() {
        Ok(output) => {
            if output.status.success() {
                Ok(extract_version_from_output(&output.stdout))
            } else {
                Ok(None)
            }
        }
        Err(e) => {
            warn!("Failed to get version for {}: {}", path, e);
            Ok(None)
        }
    }
}

/// Extract version string from command output
fn extract_version_from_output(stdout: &[u8]) -> Option<String> {
    let output_str = String::from_utf8_lossy(stdout);
    
    // Debug log the raw output
    debug!("Raw version output: {:?}", output_str);
    
    // Use regex to directly extract version pattern (e.g., "1.0.41")
    let version_regex = regex::Regex::new(r"(\d+\.\d+\.\d+(?:-[a-zA-Z0-9.-]+)?(?:\\+[a-zA-Z0-9.-]+)?)").ok()?;
    
    if let Some(captures) = version_regex.captures(&output_str) {
        if let Some(version_match) = captures.get(1) {
            let version = version_match.as_str().to_string();
            debug!("Extracted version: {:?}", version);
            return Some(version);
        }
    }
    
    debug!("No version found in output");
    None
}

/// Select the best installation based on version
pub fn select_best_installation(installations: Vec<OpenAICodexInstallation>) -> Option<OpenAICodexInstallation> {
    installations.into_iter().max_by(|a, b| {
        match (&a.version, &b.version) {
            (Some(v1), Some(v2)) => compare_versions(v1, v2),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => {
                if a.path == "openai_codex" && b.path != "openai_codex" {
                    Ordering::Less
                } else if a.path != "openai_codex" && b.path == "openai_codex" {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        }
    })
}

/// Compare two version strings
fn compare_versions(a: &str, b: &str) -> Ordering {
    let a_parts: Vec<u32> = a
        .split('.')
        .filter_map(|s|
            s.chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse()
                .ok()
        )
        .collect();

    let b_parts: Vec<u32> = b
        .split('.')
        .filter_map(|s|
            s.chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse()
                .ok()
        )
        .collect();

    for i in 0..std::cmp::max(a_parts.len(), b_parts.len()) {
        let a_val = a_parts.get(i).unwrap_or(&0);
        let b_val = b_parts.get(i).unwrap_or(&0);
        match a_val.cmp(b_val) {
            Ordering::Equal => continue,
            other => return other,
        }
    }

    Ordering::Equal
}

/// Helper function to create a Command with proper environment variables
pub fn create_command_with_env(program: &str) -> Command {
    let mut cmd = Command::new(program);
    
    info!("Creating command for: {}", program);

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
            debug!("Inheriting env var: {}={}", key, value);
            cmd.env(&key, &value);
        }
    }
    
    info!("Command will use proxy settings:");
    if let Ok(http_proxy) = std::env::var("HTTP_PROXY") {
        info!("  HTTP_PROXY={}", http_proxy);
    }
    if let Ok(https_proxy) = std::env::var("HTTPS_PROXY") {
        info!("  HTTPS_PROXY={}", https_proxy);
    }

    if program.contains("/.nvm/versions/node/") {
        if let Some(node_bin_dir) = std::path::Path::new(program).parent() {
            let current_path = std::env::var("PATH").unwrap_or_default();
            let node_bin_str = node_bin_dir.to_string_lossy();
            if !current_path.contains(&node_bin_str.as_ref()) {
                let new_path = format!("{}:{}", node_bin_str, current_path);
                debug!("Adding NVM bin directory to PATH: {}", new_path);
                cmd.env("PATH", new_path);
            }
        }
    }
    
    if program.contains("/homebrew/") || program.contains("/opt/homebrew/") {
        if let Some(program_dir) = std::path::Path::new(program).parent() {
            let current_path = std::env::var("PATH").unwrap_or_default();
            let homebrew_bin_str = program_dir.to_string_lossy();
            if !current_path.contains(&homebrew_bin_str.as_ref()) {
                let new_path = format!("{}:{}", homebrew_bin_str, current_path);
                debug!("Adding Homebrew bin directory to PATH: {}", new_path);
                cmd.env("PATH", new_path);
            }
        }
    }

    cmd
}