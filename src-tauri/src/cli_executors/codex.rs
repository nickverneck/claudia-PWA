// src-tauri/src/cli_executors/codex.rs

use anyhow::Result;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::path::PathBuf;
use std::process::Command;
use tauri::Manager;

/// Type of Codex installation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InstallationType {
    /// System-installed binary
    System,
    /// Custom path specified by user
    Custom,
}

/// Represents an Codex installation with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodexInstallation {
    /// Full path to the Codex binary
    pub path: String,
    /// Version string if available
    pub version: Option<String>,
    /// Source of discovery (e.g., "system", "homebrew", "which")
    pub source: String,
    /// Type of installation
    pub installation_type: InstallationType,
}

/// Main function to find the Codex binary
pub fn find_codex_binary(app_handle: &tauri::AppHandle) -> Result<String, String> {
    info!("Searching for codex binary...");

    // First check if we have a stored path and preference in the database
    if let Ok(app_data_dir) = app_handle.path().app_data_dir() {
        let db_path = app_data_dir.join("agents.db");
        if db_path.exists() {
            if let Ok(conn) = rusqlite::Connection::open(&db_path) {
                // Check for stored path first
                if let Ok(stored_path) = conn.query_row(
                    "SELECT value FROM app_settings WHERE key = 'codex_binary_path'",
                    [],
                    |row| row.get::<_, String>(0),
                ) {
                    info!("Found stored codex path in database: {}", stored_path);
                    
                    // Check if the path still exists
                    let path_buf = PathBuf::from(&stored_path);
                    if path_buf.exists() && path_buf.is_file() {
                        return Ok(stored_path);
                    } else {
                        warn!("Stored codex path no longer exists: {}", stored_path);
                    }
                }
                
                // Check user preference
                let preference = conn.query_row(
                    "SELECT value FROM app_settings WHERE key = 'codex_installation_preference'",
                    [],
                    |row| row.get::<_, String>(0),
                ).unwrap_or_else(|_| "system".to_string());
                
                info!("User preference for Codex installation: {}", preference);
            }
        }
    }

    // Discover all available system installations
    let installations = discover_system_installations();

    if installations.is_empty() {
        error!("Could not find codex binary in any location");
        return Err("Codex CLI not found. Please ensure it's installed and in your PATH.".to_string());
    }

    // Log all found installations
    for installation in &installations {
        info!("Found Codex installation: {:?}", installation);
    }

    // Select the best installation (highest version)
    if let Some(best) = select_best_installation(installations) {
        info!(
            "Selected Codex installation: path={}, version={:?}, source={}",
            best.path, best.version, best.source
        );
        Ok(best.path)
    } else {
        Err("No valid Codex installation found".to_string())
    }
}

/// Discovers all available Codex installations and returns them for selection
pub fn discover_codex_installations() -> Vec<CodexInstallation> {
    info!("Discovering all Codex installations...");

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
fn source_preference(installation: &CodexInstallation) -> u8 {
    match installation.source.as_str() {
        "which" => 1,
        "homebrew" => 2,
        "system" => 3,
        "local-bin" => 4,
        "PATH" => 5,
        _ => 6,
    }
}

/// Discovers all Codex installations on the system
fn discover_system_installations() -> Vec<CodexInstallation> {
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

/// Try using the 'which' command to find Codex
fn try_which_command() -> Option<CodexInstallation> {
    debug!("Trying 'which codex' to find binary...");

    match Command::new("which").arg("codex").output() {
        Ok(output) if output.status.success() => {
            let output_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

            if output_str.is_empty() {
                return None;
            }

            let path = if output_str.starts_with("codex:") && output_str.contains("aliased to") {
                output_str
                    .split("aliased to")
                    .nth(1)
                    .map(|s| s.trim().to_string())
            } else {
                Some(output_str)
            }?;

            debug!("'which' found codex at: {}", path);

            // Verify the path exists
            if !PathBuf::from(&path).exists() {
                warn!("Path from 'which' does not exist: {}", path);
                return None;
            }

            // Get version
            let version = get_codex_version(&path).ok().flatten();

            Some(CodexInstallation {
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
fn find_standard_installations() -> Vec<CodexInstallation> {
    let mut installations = Vec::new();

    // Common installation paths for codex
    let mut paths_to_check: Vec<(String, String)> = vec![
        ("/usr/local/bin/codex".to_string(), "system".to_string()),
        (
            "/opt/homebrew/bin/codex".to_string(),
            "homebrew".to_string(),
        ),
        ("/usr/bin/codex".to_string(), "system".to_string()),
        ("/bin/codex".to_string(), "system".to_string()),
    ];

    // Also check user-specific paths
    if let Ok(home) = std::env::var("HOME") {
        paths_to_check.extend(vec![
            (
                format!("{}/.local/bin/codex", home),
                "local-bin".to_string(),
            ),
        ]);
    }

    // Check each path
    for (path, source) in paths_to_check {
        let path_buf = PathBuf::from(&path);
        if path_buf.exists() && path_buf.is_file() {
            debug!("Found codex at standard path: {} ({})", path, source);

            // Get version
            let version = get_codex_version(&path).ok().flatten();

            installations.push(CodexInstallation {
                path,
                version,
                source,
                installation_type: InstallationType::System,
            });
        }
    }

    // Also check if codex is available in PATH (without full path)
    if let Ok(output) = Command::new("codex").arg("--version").output() {
        if output.status.success() {
            debug!("codex is available in PATH");
            let version = extract_version_from_output(&output.stdout);

            installations.push(CodexInstallation {
                path: "codex".to_string(),
                version,
                source: "PATH".to_string(),
                installation_type: InstallationType::System,
            });
        }
    }

    installations
}

/// Get Codex version by running --version command
fn get_codex_version(path: &str) -> Result<Option<String>, String> {
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
pub fn select_best_installation(installations: Vec<CodexInstallation>) -> Option<CodexInstallation> {
    installations.into_iter().max_by(|a, b| {
        match (&a.version, &b.version) {
            (Some(v1), Some(v2)) => compare_versions(v1, v2),
            (Some(_), None) => Ordering::Greater,
            (None, Some(_)) => Ordering::Less,
            (None, None) => {
                if a.path == "codex" && b.path != "codex" {
                    Ordering::Less
                } else if a.path != "codex" && b.path == "codex" {
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

