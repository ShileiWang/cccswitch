# 在 CC Switch 里管理 Cursor

CC Switch 现在可以直接管理 Cursor 的供应商：添加多家、一键切换，Skills 和提示词一起同步。这篇文章讲清 Cursor 的配置写在哪里、切换时发生了什么，以及它和 Claude Code、Codex 有什么不同。

## Cursor 是怎样被接管的

Cursor 一次只生效一家供应商。CC Switch 不维护「启用列表」，而是把当前选中的那家直接写进 Cursor 的配置文件，切换就是覆盖。

| 内容 | 位置 |
| --- | --- |
| 生效的供应商 | `~/.cursor/cli-config.json` |
| MCP | `~/.cursor/mcp.json` |
| Skills | `~/.cursor/skills` |
| 提示词 | `~/.cursor/rules` |
| 登录凭据 | Windows：`%APPDATA%\Cursor\auth.json`；macOS / Linux：配置目录下 |

`cli-config.json` 里 CC Switch 写的部分长这样：

```json
{
  "model": "你的模型名",
  "env": {
    "CURSOR_API_KEY": "sk-...",
    "CURSOR_BASE_URL": "https://..."
  }
}
```

文件里其它内容原样保留，只覆盖这几项。

## 添加和切换供应商

1. 侧栏选到 **Cursor**，在供应商页「添加」一家。
2. 填 API Key、端点（Base URL）和模型名。带预设的供应商可以直接选预设。
3. 保存后，在卡片上点「启用」，这一家就写进 `cli-config.json`，Cursor 下一次请求就走它。
4. 要换一家，启用另一张卡片即可。切换是覆盖，所以不需要先停用原来那家。

表单里改完保存，会立刻回写到 `cli-config.json`；Cursor 里打开新的对话就能用上。已经打开的会话仍用旧的连接，重启会话生效。

## 改配置目录

默认目录是 `~/.cursor`（macOS / Linux）。如果你的 Cursor 装在别的地方，或者想在多个目录之间分开管：

- 在 **设置 → Cursor 配置目录** 里填一个路径；
- 或者设置环境变量 `CURSOR_CONFIG_DIR`。

设置里的值优先。改了目录之后，`cli-config.json`、`mcp.json`、`skills`、`rules` 都跟着走新目录。

## Skills、提示词和 MCP

- **Skills** 装进 `~/.cursor/skills`，Cursor 自己会读这个目录。
- **提示词** 写进 `~/.cursor/rules`，和 Cursor 读 `AGENTS.md` 的目录是同一个。
- **MCP** 直接写 `~/.cursor/mcp.json`，**不出现在统一 MCP 面板里**。Cursor 只读这一份本地文件，不参与全局 MCP 同步，所以统一面板里没有 Cursor 这一列。

## 关于本地路由

Cursor **不经过** CC Switch 的本地路由服务，也没有路由模式：

- 设置 → 本地路由的「全局路由」总开关只作用于 Claude Code、Codex、Gemini、Grok Build、Claude Desktop，Cursor 不在其中；
- Cursor 的应用页里没有「路由」页签，不需要（也无法）把它接上代理。

也就是说，用 Cursor 时请求按供应商配置直连，不走本机代理，用量统计里也不会出现每请求的明细。

## 常见问题

**切换之后 Cursor 里还是旧模型？** 关掉当前会话重新开一个。`cli-config.json` 已经改了，Cursor 在会话开始时读取。

**想用 Cursor 官方登录？** 不要填 API Key，让凭据留在 Cursor 自己的 `auth.json` 里；CC Switch 启用的是你自己添加的供应商，两者互不覆盖。

**改了配置目录，之前的供应商还在吗？** 它们留在原来的目录里，没有搬走。切回原目录就能看到。
