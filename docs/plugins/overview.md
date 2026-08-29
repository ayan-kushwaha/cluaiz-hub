# ⚡ Plugins: In-Process Sandboxed Execution Muscle

In Cluaiz, **Plugins** provide the raw **Computational Muscle**. While Skills inject cognitive context, Plugins execute deterministic compiled logic (WASM or Native binaries) in-process at sub-millisecond speeds.

---

## 🎯 1. What is a Plugin?

A Plugin is an execution unit containing:
- **`manifest-plugin.yaml`**: Manifest declaring resource limits, envelope type (`WASM` / `NATIVE`), and triggers.
- **Compiled Binary (`logic.wasm` or `.dll`/`.so`)**: Compiled bytecode implementing the Cluaiz C-ABI.
- **`assets/icon.svg`**: Vector SVG icon for UI.
- **`package.json`**: Cluaiz Hub registry index.

```
plugins/my-plugin/
├── manifest-plugin.yaml  ← RAM/CPU caps, envelope, triggers
├── logic.wasm            ← Compiled WASM bytecode
├── package.json          ← Hub registry packaging
└── assets/
    └── icon.svg          ← Vector UI icon
```

---

## 🔒 2. Sandboxing Architecture (Wasmtime)

Plugins are executed via `inference_cel::WasmExecutor` with strict security enforcement:

```mermaid
flowchart LR
    A["Inference Stream"] --> B["<TRIGGER:plugin:math>"]
    B --> C["WasmExecutor::execute_with_rules"]
    C --> D["Store<PluginStoreLimits>"]
    D --> E["ResourceLimiter (Hard RAM Cap)"]
    D --> F["Fuel Consumed (Instruction Limit)"]
    D --> G["Host ABI (cluaiz:now_utc_ms)"]
    G --> H["Result String Returned in ~1ms"]
```

### Security Guarantees:
1. **Zero Ambient Authority:** A WASM plugin cannot access the host filesystem, network sockets, or environment variables unless explicitly granted.
2. **Deterministic Execution:** Eliminates LLM calculation flaws (e.g. arithmetic errors, string parsing mistakes).
3. **Hard Memory Caps:** Physical RAM is capped via `wasmtime::ResourceLimiter`. Any attempt to grow memory past `max_memory_mb` is terminated immediately.
4. **Fuel Limits:** CPU instructions are counted. Infinite loops terminate automatically when fuel is exhausted.
