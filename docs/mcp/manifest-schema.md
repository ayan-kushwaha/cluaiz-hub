# 📋 MCP Manifest Specification (`manifest-mcp.yaml`)

This document defines the complete schema specification for `manifest-mcp.yaml`. Every MCP Server registered in Cluaiz Hub must contain this manifest.

---

## 📄 Complete Example Manifest

```yaml
# =====================================================================
# cluaiz MCP MANIFEST (manifest-mcp.yaml)
# =====================================================================
name: "filesystem"
version: "1.0.0"
description: "Official Model Context Protocol secure file system bridge."
author: "MCP Steering Group"
type: "mcp"

discovery:
  semantic_triggers:
    - "read file"
    - "write file"
    - "list directory"
    - "directory tree"
  cel_grammar: "use mcp::filesystem -> call_tool(...)"

activation:
  lazy_load: true
  trigger_on:
    - "on_command:use mcp::filesystem"

permissions:
  max_memory_mb: null
  max_cpu_time_ms: 30000
  network_access: false
  file_system: "read_write"

execution:
  command: "npx"
  args:
    - "-y"
    - "@modelcontextprotocol/server-filesystem"
    - "."
  env:
    NODE_ENV: "production"

execution_mode: "manual"
default_turns: 3
```

---

## 🔍 Field Reference Table

| Field | Type | Required? | Description |
|---|---|---|---|
| `name` | `string` | **Yes** | Unique package identifier (lowercase `kebab-case`). |
| `version` | `string` | **Yes** | Semver string (e.g. `"1.0.0"`). |
| `description` | `string` | **Yes** | Human-readable explanation of the MCP server's capabilities. |
| `author` | `string` | **Yes** | Author or organization name. |
| `type` | `string` | **Yes** | Must be exactly `"mcp"`. |
| `discovery.semantic_triggers` | `string[]` | **Yes** | Keywords that activate this MCP bridge during chat inference. |
| `activation.lazy_load` | `bool` | Optional | `true` = Only spawn the child process when invoked. Default: `true`. |
| `execution.command` | `string` | **Yes** | Subprocess executable binary (e.g. `"npx"`, `"node"`, `"python"`). |
| `execution.args` | `string[]` | Optional | Command-line arguments passed to the binary. |
| `execution.env` | `map<string,string>` | Optional | Environment variables injected into the child process. |
