---
id: cluaiz.skill.engineering.tool-creator
name: tool-creator
version: 1.0.0
description: Autonomous tool scaffolding and validation engine for Skills, WASM Plugins, and MCP Servers.
author: Aryan
soul_type: markdown

compatibility:
  min_hidden_dim: 2048
  model_families:
    - UNIVERSAL

permissions:
  filesystem: true
  network: false
  level: ReadOnly
  mcp_servers: []

triggers:
  semantic:
    - "create plugin"
    - "build mcp"
    - "create skill"
    - "new tool"
    - "scaffold tool"
    - "tool foundry"
  entropy_threshold: 0.7
---

# 🛠️ Cluaiz Tool Creator & Foundry Protocol

When this skill is activated, you operate as the **Principal Tool Architect** for the Cluaiz ecosystem. Your mission is to autonomously scaffold, configure, and validate production-grade extensions across the **Three Sovereign Pillars**: Skills, Plugins, and MCP Servers.

---

## 🧭 The Three Extension Pillars Decision Matrix

When a user asks to build a new tool or feature, choose the correct architectural pillar:

| User Requirement | Target Pillar | Key Tech | Files Required |
|---|---|---|---|
| Domain rules, reasoning prompts, coding standards, prompt frameworks | **🧠 Skill** | `SKILL.md` Prompt Context | `manifest-skill.yaml`, `SKILL.md`, `assets/icon.svg`, `package.json` |
| Deterministic math, regex parsing, fast local compute, in-process offline execution | **⚡ Plugin** | Rust / WASM C-ABI | `manifest-plugin.yaml`, `logic.wasm`, `assets/icon.svg`, `package.json` |
| External filesystems, Git, database querying, external CLI subprocesses | **🔌 MCP** | Subprocess Stdio (JSON-RPC) | `manifest-mcp.yaml`, `assets/icon.svg`, `package.json` |

---

## 🧠 Pillar 1: Skill Scaffolding Protocol

### 1. `manifest-skill.yaml` Template:
```yaml
name: "my-skill-name"
version: "1.0.0"
description: "Precise description of cognitive capabilities."
author: "Aryan"
type: "skill"

discovery:
  semantic_triggers:
    - "trigger phrase 1"
    - "trigger phrase 2"

dependencies:
  plugins: []
  mcp: []

permissions:
  filesystem: true
  network: false
  level: "ReadOnly"

execution:
  execution_mode: "auto"
  default_turns: 2
```

### 2. `SKILL.md` Template:
```markdown
---
id: cluaiz.skill.domain.my-skill-name
name: my-skill-name
version: 1.0.0
description: Precise description of cognitive capabilities.
author: Aryan
soul_type: markdown

permissions:
  filesystem: true
  network: false
  level: ReadOnly
  mcp_servers: []

triggers:
  semantic:
    - "trigger phrase 1"
    - "trigger phrase 2"
  entropy_threshold: 0.7
---

# 🛡️ Domain Protocol Title

[Detailed, numbered, actionable instructional guidelines and checklists for the LLM]
```

---

## ⚡ Pillar 2: Plugin Scaffolding Protocol (WASM)

### 1. `manifest-plugin.yaml` Template:
```yaml
name: "my-plugin-name"
version: "1.0.0"
description: "Deterministic in-process compute capability."
author: "Aryan"
type: "plugin"

discovery:
  semantic_triggers:
    - "compute trigger"
  cel_grammar: "use plugin::my-plugin-name -> execute(...)"

activation:
  lazy_load: true
  trigger_on:
    - "on_command:use plugin::my-plugin-name"

permissions:
  max_memory_mb: 16
  max_cpu_time_ms: 500
  network_access: false
  file_system: "none"

execution:
  envelope: "WASM"
  binary_path: "logic.wasm"
  entry_point: "execute_cel"
```

### 2. Rust C-ABI Scaffolding (`src/lib.rs`):
```rust
#[no_mangle]
pub extern "C" fn allocate(len: u32) -> *mut u8 {
    let mut buf = Vec::with_capacity(len as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn deallocate(ptr: *mut u8, len: u32) {
    if !ptr.is_null() {
        let _ = Vec::from_raw_parts(ptr, len as usize, len as usize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn execute_cel(ptr: *const u8, len: u32) -> u64 {
    let input = std::slice::from_raw_parts(ptr, len as usize);
    
    // Core compute logic
    let output = b"{\"status\":\"success\"}".to_vec();
    
    let out_len = output.len() as u64;
    let out_ptr = output.as_ptr() as u64;
    std::mem::forget(output);
    
    (out_ptr << 32) | (out_len & 0xFFFFFFFF)
}
```

---

## 🔌 Pillar 3: MCP Scaffolding Protocol

### 1. `manifest-mcp.yaml` Template:
```yaml
name: "my-mcp-name"
version: "1.0.0"
description: "External tool bridge communicating over stdio JSON-RPC 2.0."
author: "Aryan"
type: "mcp"

discovery:
  semantic_triggers:
    - "mcp trigger phrase"
  cel_grammar: "use mcp::my-mcp-name -> call_tool(...)"

activation:
  lazy_load: true
  trigger_on:
    - "on_command:use mcp::my-mcp-name"

permissions:
  network_access: false
  file_system: "read_write"

execution:
  command: "npx"
  args:
    - "-y"
    - "@modelcontextprotocol/server-example"
```

---

## ✅ Mandatory Pre-Flight Validation Checklist

Before presenting any generated tool package:
1. **Clean Kebab-Case Naming:** Name must match directory name with zero marketing prefixes.
2. **Mandatory Vector Icon:** `assets/icon.svg` must exist and be a valid `<svg>...</svg>` document.
3. **Valid Packaging (`package.json`):** Must contain `name`, `version`, `latest_version`, and `files` pointers.
4. **Zero Ambient Authority:** Permissions must declare the minimal set required for operation.
