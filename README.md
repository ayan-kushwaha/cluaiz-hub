<p align="center">
  <img src="assets/cluaiz-hub.webp" alt="The Official Registry for the Cluaiz Ecosystem." width="100%">
</p>

<h1 align="center">Cluaiz Hub</h1>

<p align="center">
  <strong>The Official Central Registry for Cluaiz Plugins, Skills, Souls, & MCP.</strong>
</p>

<p align="center">
  <a href="https://github.com/cluaiz/skills/actions"><img src="https://img.shields.io/github/actions/workflow/status/cluaiz/skills/release-skills.yml?branch=main&style=for-the-badge" alt="CI"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-Apache_2.0-blue.svg?style=for-the-badge" alt="Apache 2.0 License"></a>
</p>

---

## 🌟 What is this?

This repository (**Cluaiz Hub**) is the central registry for the Cluaiz Inference Engine. It contains everything needed to extend the core engine's capabilities, give the AI new knowledge, and execute high-performance tools.

---

## 🏗️ Core Pillars of the Cluaiz Ecosystem

### 1. 🔌 Plugins (The Execution Muscle)
A **Plugin** provides direct in-memory execution power. Plugins can be compiled as:
- **WASM Micro-Tools (`WASM`):** Fully isolated with strict CPU fuel limits and RAM caps.
- **Native Plugins (`NATIVE`):** Bare-metal C-FFI binaries (`.dll`, `.so`, `.dylib`) for deep VRAM access, database storage, and high-speed web retrieval.
- **Example:** `cluaiz-search` (Live web metasearch), `cluaiz-db` (Neural database), `cluaiz-math` (WASM calculator).

### 2. 🧠 Skills (The AI Instructions)
A **Skill** is pure workflow and context formatting (`SKILL.md`). It teaches the AI *how* and *when* to reason and call tools without containing binary code.
- **Example:** `frontend-dev` (Guides code generation conventions).

### 3. 🌐 MCP (Model Context Protocol)
Open standard external tools. Bridges external subprocesses (such as GitHub, PostgreSQL, or Brave Search) to the Cluaiz Engine over standardized JSON-RPC stdio/HTTP transports.

---

## 🔄 End-to-End Execution Flow

```mermaid
graph TD
    U["User Prompt"] --> LLM["LLM Inference Engine"]
    
    subgraph Hub["Cluaiz Hub Registry"]
        SK["Skills (SKILL.md)"]
        PL["Plugins (WASM / Native)"]
        MCP["MCP Connectors"]
    end
    
    SK -->|Injects Context| LLM
    LLM -->|Outputs CEL / Function Call| ER["Engine Router"]
    
    ER -->|WASM Sandbox / Native C-FFI| PL
    ER -->|Standard JSON-RPC| MCP
    
    PL -->|Returns Result Pointer| ER
    MCP -->|Returns JSON| ER
    
    ER -->|KV-Cache Prefix Injection| LLM
    LLM -->|Final Output| U
```

---

## 🚀 Quick Start (CLI)

Install any module from the Hub directly via the Cluaiz CLI:

```bash
# Install a Plugin (WASM or Native)
cluaiz plugin install cluaiz-search

# Install a standalone Skill
cluaiz skill install frontend-dev

# Install an MCP Server
cluaiz mcp install github
```
