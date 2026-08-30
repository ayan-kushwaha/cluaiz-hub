use serde::{Deserialize, Serialize};

#[link(wasm_import_module = "cluaiz")]
extern "C" {
    fn now_utc_ms() -> i64;
    fn os_platform() -> i32;
}

#[derive(Deserialize)]
struct SysinfoRequest {
    #[serde(default)]
    verbose: Option<bool>,
}

#[derive(Serialize)]
struct SysinfoResponse {
    status: String,
    os_platform: String,
    platform_id: i32,
    timestamp_ms: i64,
    arch: String,
    runtime: String,
    sandbox: String,
    capabilities: Vec<String>,
}

#[no_mangle]
pub extern "C" fn allocate(len: u32) -> *mut u8 {
    let mut buf = Vec::with_capacity(len as usize);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn deallocate(ptr: *mut u8, len: u32) {
    if !ptr.is_null() && len > 0 {
        let _ = Vec::from_raw_parts(ptr, len as usize, len as usize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn execute_cel(ptr: *const u8, len: u32) -> u64 {
    let input_bytes = if len > 0 && !ptr.is_null() {
        core::slice::from_raw_parts(ptr, len as usize)
    } else {
        &[]
    };

    let _req: Option<SysinfoRequest> = serde_json::from_slice(input_bytes).ok();

    let p_id = os_platform();
    let ts_ms = now_utc_ms();

    let platform_name = match p_id {
        1 => "Windows",
        2 => "macOS",
        3 => "Linux",
        _ => "Unknown / POSIX",
    };

    let response = SysinfoResponse {
        status: "success".into(),
        os_platform: platform_name.into(),
        platform_id: p_id,
        timestamp_ms: ts_ms,
        arch: if cfg!(target_arch = "wasm32") { "wasm32".into() } else { "native".into() },
        runtime: "Cluaiz Wasmtime Isolated Sandbox".into(),
        sandbox: "memory-capped & fuel-metered".into(),
        capabilities: vec![
            "host_os_probing".into(),
            "deterministic_time".into(),
            "isolated_execution".into(),
        ],
    };

    let output = serde_json::to_vec(&response)
        .unwrap_or_else(|_| Vec::from(b"{\"status\":\"error\",\"error\":\"Serialization failure\"}".as_slice()));

    let out_len = output.len() as u64;
    let out_ptr = output.as_ptr() as u64;
    core::mem::forget(output);

    (out_ptr << 32) | (out_len & 0xFFFFFFFF)
}
