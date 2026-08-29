# 🔌 WASM C-ABI & Capability Host Interface Guide

Cluaiz executes WebAssembly plugins via a high-performance, zero-overhead C-ABI interface. This document explains the mandatory exported functions and the available Host ABI hooks.

---

## 🛠️ 1. Required WASM Exported Symbols

Every WASM binary (`logic.wasm`) compiled for Cluaiz MUST export the following 3 C-ABI functions:

### 1. `allocate(len: u32) -> *mut u8`
Allocates a memory buffer of `len` bytes inside WASM linear memory for incoming payloads:
```rust
#[no_mangle]
pub extern "C" fn allocate(len: u32) -> *mut u8 {
    let mut buf = Vec::with_capacity(len as usize);
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}
```

### 2. `deallocate(ptr: *mut u8, len: u32)`
Reclaims a previously allocated memory buffer:
```rust
#[no_mangle]
pub unsafe extern "C" fn deallocate(ptr: *mut u8, len: u32) {
    if !ptr.is_null() {
        let _ = Vec::from_raw_parts(ptr, len as usize, len as usize);
    }
}
```

### 3. `execute_cel(ptr: *const u8, len: u32) -> u64`
Processes the input buffer and returns a 64-bit packed pointer `(ptr << 32) | len` pointing to the output buffer:
```rust
#[no_mangle]
pub unsafe extern "C" fn execute_cel(ptr: *const u8, len: u32) -> u64 {
    let input_bytes = std::slice::from_raw_parts(ptr, len as usize);
    
    // Process logic here...
    let output = b"{\"status\":\"success\"}".to_vec();
    
    let out_len = output.len() as u64;
    let out_ptr = output.as_ptr() as u64;
    std::mem::forget(output);
    
    (out_ptr << 32) | (out_len & 0xFFFFFFFF)
}
```

---

## 🌐 2. Cluaiz Capability-Gated Host ABI

The Cluaiz runtime binds capability-gated host hooks into the Wasmtime Linker, providing plugins with offline hardware telemetry and system time without WASI bloat:

```rust
// Linker Host Hooks exposed in "cluaiz" namespace
extern "C" {
    // Returns current UTC timestamp in milliseconds
    pub fn now_utc_ms() -> i64;
    
    // Returns OS platform code (1=Windows, 2=macOS, 3=Linux, 0=Unknown)
    pub fn os_platform() -> i32;
}
```

### Usage Example in Rust:
```rust
#[no_mangle]
pub unsafe extern "C" fn get_current_timestamp() -> i64 {
    now_utc_ms()
}
```
