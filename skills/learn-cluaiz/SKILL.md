---
name: learn-cluaiz
description: Comprehensive interactive knowledge guide to Cluaiz architecture, CEL runtime, and local execution.
version: 1.0.0
triggers:
  - "learn cluaiz"
  - "how cluaiz works"
  - "cluaiz architecture"
  - "what is cel"
execution_mode: auto
default_turns: 3
---

# 🚀 Learn Cluaiz: Architecture & Engine Mechanics

Cluaiz is a high-performance local AI inference engine designed to execute LLMs, WASM plugins, and MCP servers directly on user hardware with single-pass turn lifecycles.

## 🏗️ Core Architecture Overview

```
Client (HTTP/SSE) ──> Axum API (chat.rs) ──> ToolsEngine (Registry/Lifecycle)
                                             ├── Skills (Prompt Augmentation)
                                             ├── Plugins (CEL / Wasmtime Sandbox)
                                             └── MCP (Subprocess Stdio IPC)
```

## 🧩 The Three Extension Pillars
1. **Skills:** Lightweight markdown frameworks (`SKILL.md`) that inject system instructions and specialized protocols into model context dynamically.
2. **Plugins:** WASM and Native binaries executing in isolated memory sandboxes via `inference_cel::WasmExecutor` with strict fuel and resource limits.
3. **MCP Servers:** Subprocess IPC bridges communicating with external tool ecosystems over standard I/O via JSON-RPC 2.0.

## 🛡️ Sovereign Principles
- **100% Offline by Default:** Engine and tools run locally without external data exfiltration.
- **Strict DRY Engineering:** Modular components with zero logic duplication.
- **Single-Pass Tool Interception:** Seamlessly handles tool triggers mid-stream without multi-request overhead.
