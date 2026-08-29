# 🚀 Tutorial: Build Your First WASM Plugin

In this tutorial, we will write, compile, and package a high-performance WASM Plugin in Rust: **`hasher`** (computing SHA-256 hashes deterministically).

---

## 🛠️ Step 1: Initialize Rust WASM Project

```bash
cargo new --lib hasher
cd hasher
```

Configure `Cargo.toml`:

```toml
[package]
name = "hasher"
version = "1.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
sha2 = "0.10"
hex = "0.4"
```

---

## 💻 Step 2: Implement C-ABI in `src/lib.rs`

```rust
use sha2::{Sha256, Digest};

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
    
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();
    let hex_hash = hex::encode(result);
    
    let out_bytes = hex_hash.into_bytes();
    let out_len = out_bytes.len() as u64;
    let out_ptr = out_bytes.as_ptr() as u64;
    std::mem::forget(out_bytes);
    
    (out_ptr << 32) | (out_len & 0xFFFFFFFF)
}
```

---

## 🔨 Step 3: Compile to WebAssembly

```bash
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/hasher.wasm logic.wasm
```

---

## 📄 Step 4: Create `manifest-plugin.yaml`

```yaml
name: "hasher"
version: "1.0.0"
description: "Deterministic SHA-256 cryptographic hashing plugin."
author: "Aryan"
type: "plugin"

discovery:
  semantic_triggers:
    - "sha256"
    - "hash text"
    - "generate hash"

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

---

## 🎨 Step 5: Add `assets/icon.svg` & `package.json`

Add `assets/icon.svg` and `package.json`. Copy the folder to `~/.cluaiz/tools/plugins/hasher/`. The engine will auto-probe and make the plugin available for inference immediately!
