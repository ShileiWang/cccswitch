//! Cursor 的原生配置目录与凭据文件。
//!
//! Cursor 本身不是 additive 应用：CC Switch 只写一个 provider 的 live 配置，切换就是覆盖。
//! 它走不走本地代理（`supports_local_proxy() = false`），也没有 route mode，所以这里只负责
//! 路径解析和 `~/.cursor/mcp.json` 的读写，真正的 provider 落盘由 `services::provider::live`
//! 统一处理。
//!
//! 平台约定（和 cursor-byok 保持一致）：
//! - 配置目录：macOS/Linux 为 `~/.cursor`；Windows 认证文件在 `%APPDATA%\Cursor\auth.json`。
//! - CLI 用 `CURSOR_AUTH_TOKEN` 环境变量接管登录态。

use std::fs;
use std::path::PathBuf;

use serde_json::{json, Value};

use crate::config::{get_home_dir, write_json_file};
use crate::error::AppError;

/// `~/.cursor`：mcp.json / cli-config.json / rules 都在这里。
pub fn cursor_dir() -> PathBuf {
    if let Some(dir) = crate::settings::get_cursor_override_dir() {
        return dir;
    }
    std::env::var_os("CURSOR_CONFIG_DIR")
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| get_home_dir().join(".cursor"))
}

/// `~/.cursor/mcp.json`：Cursor 的 MCP 服务器表（`{ "mcpServers": { ... } }`）。
pub fn mcp_path() -> PathBuf {
    cursor_dir().join("mcp.json")
}

/// CLI 读的 `~/.cursor/cli-config.json`。
pub fn cli_config_path() -> PathBuf {
    cursor_dir().join("cli-config.json")
}

/// 读取 MCP 配置；文件不存在或为空时给出空表，坏 JSON 才报错。
pub fn read_mcp() -> Result<Value, AppError> {
    read_json_or_empty(&mcp_path())
}

/// 覆盖写 MCP 配置，保证 `mcpServers` 是对象。
pub fn write_mcp(value: &Value) -> Result<(), AppError> {
    let mut value = value.clone();
    if !value.is_object() {
        return Err(AppError::Config(
            "Cursor MCP configuration must be an object".into(),
        ));
    }
    if value.get("mcpServers").is_none_or(|v| !v.is_object()) {
        value["mcpServers"] = json!({});
    }
    let path = mcp_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| AppError::io(parent, e))?;
    }
    write_json_file(&path, &value)
}

/// Cursor 的登录凭据文件。Windows 在 `%APPDATA%\Cursor\auth.json`，其他平台在配置目录下。
pub fn cursor_auth_path() -> PathBuf {
    #[cfg(windows)]
    {
        if let Some(appdata) = std::env::var_os("APPDATA").filter(|value| !value.is_empty()) {
            return PathBuf::from(appdata).join("Cursor").join("auth.json");
        }
    }
    #[cfg(target_os = "macos")]
    {
        return get_home_dir().join(".cursor").join("auth.json");
    }
    #[cfg(not(any(windows, target_os = "macos")))]
    {
        let base = std::env::var_os("XDG_CONFIG_HOME")
            .filter(|value| !value.is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(|| get_home_dir().join(".config"));
        return base.join("cursor").join("auth.json");
    }
    #[allow(unreachable_code)]
    cursor_dir().join("auth.json")
}

/// `~/.cursor/skills`：Cursor 读 Skills 的目录（与 MCP 同根）。
pub fn skills_dir() -> PathBuf {
    cursor_dir().join("skills")
}

fn read_json_or_empty(path: &PathBuf) -> Result<Value, AppError> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(json!({})),
        Err(error) => return Err(AppError::io(path, error)),
    };
    if text.trim().is_empty() {
        return Ok(json!({}));
    }
    serde_json::from_str(&text)
        .map_err(|e| AppError::Config(format!("Invalid Cursor configuration {}: {e}", path.display())))
}

/// 供应商在 `cli-config.json` 里的落点：`{ "models": { ... }, "env": { ... } }` 之外没有
/// 官方的 provider 表，所以这里按 cursor-byok 的做法把直连信息写成
/// `{ "model": <名字>, "env": { "CURSOR_API_KEY", "CURSOR_BASE_URL" } }`，
/// 登录态则交回 CLI 自己的 `auth.json`。
const PROVIDER_ROOT_KEY: &str = "models";

/// 把 CC Switch 的供应商配置投影成 Cursor 的 live 片段。
pub fn provider_fragment(settings_config: &Value) -> Value {
    let env = settings_config.get("env");
    let model = settings_config
        .pointer("/config/model")
        .or_else(|| settings_config.get("model"))
        .cloned()
        .filter(|value| !value.is_null());
    let mut fragment = serde_json::Map::new();
    if let Some(model) = model {
        fragment.insert("model".into(), model);
    }
    if let Some(env) = env {
        fragment.insert("env".into(), env.clone());
    }
    Value::Object(fragment)
}

/// 供应商配置里填的直连凭据。空表表示这份配置不带 Key（官方登录态）。
pub fn provider_credentials(settings_config: &Value) -> (Option<String>, Option<String>) {
    let env = settings_config.get("env");
    let key = env
        .and_then(|env| env.get("CURSOR_API_KEY").or_else(|| env.get("ANTHROPIC_API_KEY")))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);
    let base_url = env
        .and_then(|env| {
            env.get("CURSOR_BASE_URL")
                .or_else(|| env.get("ANTHROPIC_BASE_URL"))
        })
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);
    (key, base_url)
}

/// 读回 Cursor 的 live 片段（`cli-config.json` + 凭据），给编辑表单回填。
pub fn read_live_settings() -> Result<Value, AppError> {
    let config = read_json_or_empty(&cli_config_path())?;
    let (key, base_url) = provider_credentials(&config);
    let model = config.get("model").cloned().filter(|v| !v.is_null());
    let mut env = serde_json::Map::new();
    if let Some(key) = key {
        env.insert("CURSOR_API_KEY".into(), json!(key));
    }
    if let Some(base_url) = base_url {
        env.insert("CURSOR_BASE_URL".into(), json!(base_url));
    }
    Ok(json!({
        "env": env,
        "config": {
            "model": model.unwrap_or(Value::Null),
            PROVIDER_ROOT_KEY: config.get(PROVIDER_ROOT_KEY).cloned().unwrap_or(json!({})),
        }
    }))
}

/// 判断 live 片段里是否真的有一份供应商配置（有 Key / 端点 / 模型任一）。
/// 空的 `cli-config.json` 不是用户的供应商，导入时应跳过。
pub fn has_managed_content(settings: &Value) -> bool {
    let (key, base_url) = provider_credentials(settings);
    let model = settings
        .pointer("/config/model")
        .or_else(|| settings.get("model"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty());
    key.is_some() || base_url.is_some() || model.is_some()
}

/// 把供应商切成当前：整份覆盖写 `cli-config.json`（Cursor 是切换式应用，一次只活一个
/// provider）。
pub fn set_provider(settings_config: &Value) -> Result<(), AppError> {
    let path = cli_config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| AppError::io(parent, e))?;
    }
    let mut document = read_json_or_empty(&path)?;
    let fragment = provider_fragment(settings_config);
    if let (Some(document), Some(fragment)) = (document.as_object_mut(), fragment.as_object()) {
        for (key, value) in fragment {
            document.insert(key.clone(), value.clone());
        }
    }
    write_json_file(&path, &document)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_paths_share_one_root() {
        let dir = cursor_dir();
        assert_eq!(mcp_path().parent(), Some(dir.as_path()));
        assert_eq!(cli_config_path().parent(), Some(dir.as_path()));
        assert_eq!(skills_dir().parent(), Some(dir.as_path()));
    }
}
