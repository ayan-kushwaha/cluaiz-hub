# 📋 Skill Manifest Specification (`manifest-skill.yaml`)

This document defines the complete schema specification for `manifest-skill.yaml`. Every Skill published to Cluaiz Hub must contain this manifest.

---

## 📄 Complete Example Manifest

```yaml
# =====================================================================
# cluaiz SKILL MANIFEST (manifest-skill.yaml)
# =====================================================================
name: "code-reviewer"
version: "1.0.0"
description: "Strict multi-language code quality, security vulnerability, memory safety, and DRY enforcement protocol."
author: "Aryan"
type: "skill"

discovery:
  semantic_triggers:
    - "review this code"
    - "audit security"
    - "check bugs"
    - "code quality audit"
    - "dry check"

dependencies:
  plugins:
    - "text"
  mcp:
    - "git"

permissions:
  filesystem: true
  network: false
  level: "ReadOnly"

execution:
  execution_mode: "auto" # "auto" | "manual"
  default_turns: 2       # Number of turns this skill remains active
```

---

## 🔍 Field Reference Table

| Field | Type | Required? | Description |
|---|---|---|---|
| `name` | `string` | **Yes** | Unique package identifier (lowercase `kebab-case`). Must match directory name. |
| `version` | `string` | **Yes** | Semver version string (e.g. `"1.0.0"`). |
| `description` | `string` | **Yes** | Human-readable explanation of the skill's purpose. |
| `author` | `string` | **Yes** | Author or organization name. |
| `type` | `string` | **Yes** | Must be exactly `"skill"`. |
| `discovery.semantic_triggers` | `string[]` | **Yes** | List of keyword phrases that activate this skill during inference. |
| `dependencies.plugins` | `string[]` | Optional | List of WASM plugins required by this skill. |
| `dependencies.mcp` | `string[]` | Optional | List of MCP servers required by this skill. |
| `permissions.filesystem` | `bool` | Optional | Whether the skill requires workspace file reading. |
| `permissions.network` | `bool` | Optional | Whether external network access is needed. |
| `permissions.level` | `string` | Optional | `"ReadOnly"`, `"ReadWrite"`, or `"None"`. |
| `execution.execution_mode` | `string` | Optional | `"auto"` (activated by triggers) or `"manual"` (user attached). Default: `"auto"`. |
| `execution.default_turns` | `integer` | Optional | Lifetime turn count before auto-purging (`-1` for persistent). Default: `1`. |
