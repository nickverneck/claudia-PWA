use crate::core_logic::proxy::{self as core_proxy, ProxySettings};
use tauri::{command, State};
use crate::commands::agents::AgentDb;

#[command]
pub async fn get_proxy_settings(db: State<'_, AgentDb>) -> Result<ProxySettings, String> {
    core_proxy::get_proxy_settings(&db)
}

#[command]
pub async fn save_proxy_settings(
    db: State<'_, AgentDb>,
    settings: ProxySettings,
) -> Result<(), String> {
    core_proxy::save_proxy_settings(&db, settings)
}
