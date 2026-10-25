//! Cursor 的 MCP 同步：`~/.cursor/mcp.json`，顶层 `mcpServers`。
//!
//! 格式与 Claude / Pi 相同（`{ "mcpServers": { <id>: { command, args, env, ... } } }`），
//! 只是没有 `enabled` / `disabledServers` 这类原生开关：勾选即写入条目，取消勾选即删条目。

use crate::error::AppError;
use serde_json::{json, Value};
use std::sync::Mutex;

use super::validation;

static WRITE_LOCK: Mutex<()> = Mutex::new(());

/// 写入（或覆盖）一个服务器条目。
pub fn sync_single_server_to_cursor(id: &str, spec: &Value) -> Result<(), AppError> {
    let _guard = lock()?;
    let mut document = crate::cursor_config::read_mcp()?;
    validation::validate_server_spec(spec)?;
    let servers = servers_mut(&mut document)?;
    servers.insert(id.to_string(), spec.clone());
    crate::cursor_config::write_mcp(&document)
}

/// 取消勾选 / 删除：Cursor 没有禁用态，直接摘掉条目。
pub fn remove_server_from_cursor(id: &str) -> Result<(), AppError> {
    let _guard = lock()?;
    let mut document = crate::cursor_config::read_mcp()?;
    let servers = servers_mut(&mut document)?;
    if servers.remove(id).is_none() {
        return Ok(());
    }
    crate::cursor_config::write_mcp(&document)
}

fn lock() -> Result<std::sync::MutexGuard<'static, ()>, AppError> {
    WRITE_LOCK
        .lock()
        .map_err(|e| AppError::Message(e.to_string()))
}

fn servers_mut(document: &mut Value) -> Result<&mut serde_json::Map<String, Value>, AppError> {
    document
        .as_object_mut()
        .ok_or_else(|| AppError::Config("Cursor MCP configuration must be an object".into()))?
        .entry("mcpServers")
        .or_insert_with(|| json!({}))
        .as_object_mut()
        .ok_or_else(|| AppError::Config("Cursor MCP 'mcpServers' must be an object".into()))
}
