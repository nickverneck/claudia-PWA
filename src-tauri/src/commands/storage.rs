use crate::core_logic::storage::{self as core_storage, TableInfo, TableData, QueryResult};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use tauri::{AppHandle, command, State};
use crate::commands::agents::AgentDb;

#[command]
pub async fn storage_list_tables(db: State<'_, AgentDb>) -> Result<Vec<TableInfo>, String> {
    core_storage::list_tables(&db)
}

#[command]
#[allow(non_snake_case)]
pub async fn storage_read_table(
    db: State<'_, AgentDb>,
    tableName: String,
    page: i64,
    pageSize: i64,
    searchQuery: Option<String>,
) -> Result<TableData, String> {
    core_storage::read_table(&db, tableName, page, pageSize, searchQuery)
}

#[command]
#[allow(non_snake_case)]
pub async fn storage_update_row(
    db: State<'_, AgentDb>,
    tableName: String,
    primaryKeyValues: HashMap<String, JsonValue>,
    updates: HashMap<String, JsonValue>,
) -> Result<(), String> {
    core_storage::update_row(&db, tableName, primaryKeyValues, updates)
}

#[command]
#[allow(non_snake_case)]
pub async fn storage_delete_row(
    db: State<'_, AgentDb>,
    tableName: String,
    primaryKeyValues: HashMap<String, JsonValue>,
) -> Result<(), String> {
    core_storage::delete_row(&db, tableName, primaryKeyValues)
}

#[command]
#[allow(non_snake_case)]
pub async fn storage_insert_row(
    db: State<'_, AgentDb>,
    tableName: String,
    values: HashMap<String, JsonValue>,
) -> Result<i64, String> {
    core_storage::insert_row(&db, tableName, values)
}

#[command]
pub async fn storage_execute_sql(
    db: State<'_, AgentDb>,
    query: String,
) -> Result<QueryResult, String> {
    core_storage::execute_sql(&db, query)
}

#[command]
pub async fn storage_reset_database(app: AppHandle) -> Result<(), String> {
    core_storage::reset_database(&app)
}