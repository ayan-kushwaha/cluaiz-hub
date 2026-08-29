# 🚀 Tutorial: Connect an MCP Server to Cluaiz

In this tutorial, we will configure, package, and test an official Model Context Protocol server in Cluaiz: **`everything`** (the official MCP reference test target).

---

## 🛠️ Step 1: Create Directory Structure

```bash
mkdir -p everything/assets
cd everything
```

---

## 📄 Step 2: Create `manifest-mcp.yaml`

Create `manifest-mcp.yaml`:

```yaml
name: "everything"
version: "1.0.0"
description: "Official Model Context Protocol reference test server exercising all MCP protocol capabilities."
author: "MCP Steering Group"
type: "mcp"

discovery:
  semantic_triggers:
    - "test mcp"
    - "mcp protocol test"
    - "everything server"

permissions:
  network_access: true
  file_system: "none"

execution:
  command: "npx"
  args:
    - "-y"
    - "@modelcontextprotocol/server-everything"
```

---

## 🎨 Step 3: Add `assets/icon.svg` & `package.json`

Add `assets/icon.svg`:

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="24" height="24" fill="none" stroke="#8B5CF6" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
  <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
</svg>
```

Add `package.json`:

```json
{
  "name": "everything",
  "version": "1.0.0",
  "description": "Official Model Context Protocol reference test server",
  "category": "mcp",
  "author": "MCP Steering Group",
  "license": "MIT",
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "files": {
        "manifest": "manifest-mcp.yaml",
        "icon": "assets/icon.svg"
      }
    }
  }
}
```

---

## 🧪 Step 4: Test in Cluaiz

Copy the directory to `~/.cluaiz/tools/mcp/everything/`. When prompted with `"test mcp"`, the Cluaiz engine will spawn the subprocess, exchange JSON-RPC 2.0 frames over stdio, and return results seamlessly!
