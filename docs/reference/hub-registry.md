# 🏛️ Cluaiz Hub Registry Architecture & Package Packaging

This document defines how packages are indexed, versioned, and distributed across the decentralized Cluaiz Hub repository.

---

## 🗂️ 1. Registry Topology & File Hierarchy

```
cluaiz-hub/
├── registry.json             ← Root Router
├── skills/
│   ├── family.json           ← Index of all published Skills
│   └── code-reviewer/
│       ├── package.json      ← Package version mapping & asset hashes
│       ├── manifest-skill.yaml
│       ├── SKILL.md
│       └── assets/icon.svg
├── plugins/
│   ├── family.json           ← Index of all published Plugins
│   └── math/
│       ├── package.json
│       ├── manifest-plugin.yaml
│       ├── logic.wasm
│       └── assets/icon.svg
└── mcp/
    ├── family.json           ← Index of all published MCP Servers
    └── filesystem/
        ├── package.json
        ├── manifest-mcp.yaml
        └── assets/icon.svg
```

---

## 📜 2. Master Routing (`registry.json`)

The root `registry.json` directs the CLI and Engine installer to the corresponding family catalogs:

```json
{
  "version": "1.0.0",
  "routing": {
    "skills": "skills/family.json",
    "plugins": "plugins/family.json",
    "mcp": "mcp/family.json"
  }
}
```

---

## 📦 3. Family Index (`family.json`)

Each category directory contains a `family.json` mapping package names to their `package.json` relative paths:

```json
{
  "category": "plugins",
  "name": "Cluaiz Official Plugins",
  "items": {
    "math": "math/package.json",
    "text": "text/package.json",
    "search": "search/package.json",
    "time": "time/package.json",
    "sysinfo": "sysinfo/package.json"
  }
}
```

---

## 📋 4. Package Metadata (`package.json`)

Every package must contain a `package.json` defining semver versions and file bundle links:

```json
{
  "name": "math",
  "version": "1.0.0",
  "description": "Deterministic high-precision arithmetic and mathematical evaluation engine",
  "category": "plugin",
  "author": "Aryan",
  "license": "Apache-2.0",
  "latest_version": "1.0.0",
  "versions": {
    "1.0.0": {
      "files": {
        "manifest": "manifest-plugin.yaml",
        "binary": "logic.wasm",
        "icon": "assets/icon.svg"
      }
    }
  }
}
```

---

## 🎨 5. Mandatory UI Vector Asset (`assets/icon.svg`)

Every tool in Cluaiz Hub MUST contain a valid standalone SVG file (`assets/icon.svg`):
- Format: Standard `<svg>...</svg>` vector document.
- Dimensions: 24x24 viewBox.
- The Cluaiz Web UI and desktop applications render this SVG directly inline for a native look.
