# 🧩 cluaiz Math Plugin (WASM Sandbox Demo)

This plugin is the **WASM Sandboxed** dynamic plugin template for Cluaiz.

## Overview
Unlike `cluaiz-db` (which is a native compiled `.dll` for low-latency memory operations), `cluaiz-math` represents a **Sandboxed Plugin** designed for safety and ease of distribution.

- **No Local Compilation:** The host engine loads a pre-compiled WebAssembly bytecode (`bin/plugin.wasm`) directly into the sandboxed Wasmtime engine.
- **Strict Isolation:** The execution is capped at a memory limit and fuel budget dynamically enforced by the engine rules to guarantee zero memory leaks or host crash vulnerability.

## How it works (The WASM Flow)
1. The user installs the plugin containing `manifest-plugin.yaml` and `bin/plugin.wasm` directly into `.cluaiz/plugins/`.
2. Upon engine boot, `MasterRegistry` parses `manifest-plugin.yaml` and checks that the `envelope` is `WASM`.
3. The engine initializes a safe `WasmExecutor` context, loads the `bin/plugin.wasm` file dynamically, and routes matching CEL expressions directly through the WebAssembly FFI interface.
4. **No local compiler, Cargo setup, or submodules are required.**
