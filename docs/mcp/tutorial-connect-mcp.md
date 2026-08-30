# 🚀 Tutorial: Connect an MCP Server to Cluaiz

In this tutorial, we will configure, package, and test an official Model Context Protocol server in Cluaiz: **`everything`** (the official MCP reference test target).

---

## 🛠️ Step 1: Create Directory Structure

```bash
mkdir -p everything/assets
cd everything
```

---

## 📦 Step 2: Create `package.json`

Create `package.json` with the subprocess command and arguments:

```json
{
  "id": "everything",
  "name": "MCP Everything Server",
  "category": "mcp",
  "hub_type": "mcp",
  "logo": "/assets/icon.svg",
  "title": "Model Context Protocol Reference Server",
  "description": "Official reference MCP server exercising full tool, resource, and prompt protocols.",
  "author": {
    "name": "MCP Steering Group",
    "url": "https://modelcontextprotocol.io"
  },
  "license": "MIT",
  "tags": [
    "MCP",
    "Reference",
    "Bridge"
  ],
  "dependencies": {
    "plugins": {},
    "mcp": {},
    "skills": {}
  },
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "updated_at": "2026-07-01T12:00:00Z",
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-everything"
      ],
      "files": {
        "icon": "/assets/icon.svg"
      }
    }
  }
}
```

---

## 🎨 Step 3: Add `assets/icon.svg`

Add `assets/icon.svg`:

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24" fill="none" stroke="#8B5CF6" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
</svg>
```

---

## 🧪 Step 4: Test in Cluaiz

Copy the directory to `~/.cluaiz/tools/mcp/everything/`. When prompted with `"test mcp"`, the Cluaiz engine will spawn the subprocess, exchange JSON-RPC 2.0 frames over stdio, and return results seamlessly!
