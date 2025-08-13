'''use crate::core_logic::usage::{self as core_usage, UsageEntry, UsageStats, ProjectUsage};
use tauri::command;

#[command]
pub fn get_usage_stats(days: Option<u32>) -> Result<UsageStats, String> {
    core_usage::get_usage_stats(days)
}

#[command]
pub fn get_usage_by_date_range(start_date: String, end_date: String) -> Result<UsageStats, String> {
    core_usage::get_usage_by_date_range(start_date, end_date)
}

#[command]
pub fn get_usage_details(
    project_path: Option<String>,
    date: Option<String>,
) -> Result<Vec<UsageEntry>, String> {
    core_usage::get_usage_details(project_path, date)
}

#[command]
pub fn get_session_stats(
    since: Option<String>,
    until: Option<string>,
    order: Option<String>,
) -> Result<Vec<ProjectUsage>, String> {
    core_usage::get_session_stats(since, until, order)
}
''