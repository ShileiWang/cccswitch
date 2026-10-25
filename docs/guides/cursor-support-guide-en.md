# Managing Cursor in CC Switch

CC Switch now manages Cursor's providers directly: add several, switch with one click, and keep Skills and prompt files in sync. This guide covers where Cursor's config lives, what happens when you switch, and how it differs from Claude Code and Codex.

## How Cursor is managed

Cursor runs one provider at a time. CC Switch does not keep an "enabled list" for it; switching overwrites the live provider config.

| What | Where |
| --- | --- |
| Active provider | `~/.cursor/cli-config.json` |
| MCP | `~/.cursor/mcp.json` |
| Skills | `~/.cursor/skills` |
| Prompt files | `~/.cursor/rules` |
| Login credentials | Windows: `%APPDATA%\Cursor\auth.json`; macOS / Linux: in the config directory |

The part CC Switch writes in `cli-config.json` looks like this:

```json
{
  "model": "your-model-name",
  "env": {
    "CURSOR_API_KEY": "sk-...",
    "CURSOR_BASE_URL": "https://..."
  }
}
```

Everything else in the file is left untouched; only these keys are overwritten.

## Adding and switching providers

1. Select **Cursor** in the sidebar and add a provider on its page.
2. Fill in the API key, base URL and model name. Providers with a preset can use it directly.
3. After saving, click **Enable** on the card. That provider is written to `cli-config.json` and Cursor's next request goes to it.
4. To change provider, enable another card. Switching is an overwrite, so you do not need to disable the previous one first.

Edits in the form are written back to `cli-config.json` immediately; a new conversation in Cursor picks them up. Conversations already open keep the old connection until restarted.

## Changing the config directory

The default is `~/.cursor` (macOS / Linux). If your Cursor uses another location, or you want separate setups:

- Set a path in **Settings → Cursor configuration directory**; or
- Set the `CURSOR_CONFIG_DIR` environment variable.

The setting wins. After changing it, `cli-config.json`, `mcp.json`, `skills` and `rules` all follow the new directory.

## Skills, prompt files and MCP

- **Skills** are installed into `~/.cursor/skills`, which Cursor reads itself.
- **Prompt files** go into `~/.cursor/rules`, the same directory Cursor reads `AGENTS.md` from.
- **MCP** is written straight to `~/.cursor/mcp.json` and is **not** part of the unified MCP panel. Cursor reads only this one local file and does not take part in global MCP sync, so there is no Cursor column in that panel.

## About local routing

Cursor does **not** go through CC Switch's local routing service and has no routing mode:

- The **global routing** switch in Settings → local routing applies to Claude Code, Codex, Gemini, Grok Build and Claude Desktop only;
- Cursor's page has no Routing tab, and it cannot be attached to the proxy.

In practice, Cursor connects straight to the provider you enabled. Requests do not pass through the local proxy, and Usage shows no per-request detail for them.

## FAQ

**Cursor still shows the old model after switching?** Close the conversation and start a new one. `cli-config.json` has changed; Cursor reads it when a conversation starts.

**Can I keep using Cursor's official login?** Yes, leave the API key empty so credentials stay in Cursor's own `auth.json`. CC Switch enables the providers you added; neither overwrites the other.

**I changed the config directory; are my old providers gone?** No, they stay in the old directory. Switch back to see them.
