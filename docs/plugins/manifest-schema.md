# 📋 Plugin Manifest Specification (`manifest-plugin.yaml`)

This document defines the complete schema specification for `manifest-plugin.yaml`. Every Plugin published to Cluaiz Hub must contain this manifest.

---

## 📄 Complete Example Manifest

```yaml
# =====================================================================
# cluaiz PLUGIN MANIFEST (manifest-plugin.yaml)
# =====================================================================
name: "math"
version: "1.0.0"
description: "Deterministic high-precision arithmetic, calculus, and mathematical evaluator."
author: "Aryan"
type: "plugin"

discovery:
  semantic_triggers:
    - "calculate"
    - "math"
    - "solve equation"
    - "trigonometry"
    - "evaluate math"
  cel_grammar: "use plugin::math -> evaluate(...)"

activation:
  lazy_load: true
  trigger_on:
    - "on_command:use plugin::math"

permissions:
  max_memory_mb: 16
  max_cpu_time_ms: 500
  network_access: false
  vram_kv_inject: false
  prefix_caching: false
  file_system: "none" # "none" | "read_only" | "read_write"

execution:
  envelope: "WASM"     # "WASM" | "NATIVE"
  binary_path: "logic.wasm"
  entry_point: "execute_cel"
  payload_format: "MsgPack"

execution_mode: "auto" # "auto" | "manual"
default_turns: 1
```

---

## 🔍 Field Reference Table

| Field | Type | Required? | Description |
|---|---|---|---|
| `name` | `string` | **Yes** | Unique plugin name (lowercase `kebab-case`). Must match folder name. |
| `version` | `string` | **Yes** | Semver string (e.g. `"1.0.0"`). |
| `description` | `string` | **Yes** | Human-readable explanation of the plugin's compute purpose. |
| `author` | `string` | **Yes** | Author or organization name. |
| `type` | `string` | **Yes** | Must be exactly `"plugin"`. |
| `discovery.semantic_triggers` | `string[]` | **Yes** | Keywords that activate this plugin during inference stream. |
| `discovery.cel_grammar` | `string` | Optional | CEL invocation expression format. |
| `activation.lazy_load` | `bool` | Optional | `true` = Only load into RAM when invoked. Default: `true`. |
| `permissions.max_memory_mb` | `integer` | Optional | Hard RAM limit enforced via `ResourceLimiter`. |
| `permissions.max_cpu_time_ms` | `integer` | Optional | Max CPU execution time before fuel timeout. |
| `permissions.network_access` | `bool` | Optional | Whether HTTP calls are allowed in sandbox. |
| `permissions.file_system` | `string` | Optional | `"none"`, `"read_only"`, or `"read_write"`. |
| `execution.envelope` | `string` | **Yes** | `"WASM"` (sandboxed bytecode) or `"NATIVE"` (trusted C-FFI). |
| `execution.binary_path` | `string` | **Yes** | Path to the compiled binary (e.g. `"logic.wasm"`). |
| `execution.entry_point` | `string` | Optional | Exported function symbol. Default: `"execute_cel"`. |
