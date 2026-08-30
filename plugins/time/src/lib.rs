extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

// Host ABI exposed by Cluaiz Wasmtime Linker
#[link(wasm_import_module = "cluaiz")]
extern "C" {
    fn now_utc_ms() -> i64;
    fn os_platform() -> i32;
}

#[derive(Deserialize)]
struct TimeRequest {
    #[serde(default)]
    format: Option<String>,
}

#[derive(Serialize)]
struct TimeResponse {
    status: String,
    timestamp_ms: i64,
    iso_8601: String,
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
    if !ptr.is_null() {
        let _ = Vec::from_raw_parts(ptr, len as usize, len as usize);
    }
}

#[no_mangle]
pub unsafe extern "C" fn execute_cel(ptr: *const u8, len: u32) -> u64 {
    let _input_slice = if len > 0 && !ptr.is_null() {
        core::slice::from_raw_parts(ptr, len as usize)
    } else {
        &[]
    };

    let ts_ms = now_utc_ms();
    let total_secs = ts_ms / 1000;
    
    // Convert timestamp to approximate ISO-8601 string without external heavy crate
    let days = total_secs / 86400;
    let time_in_day = total_secs % 86400;
    let hours = time_in_day / 3600;
    let minutes = (time_in_day % 3600) / 60;
    let seconds = time_in_day % 60;

    // Simple Gregorian date calculation from epoch (1970-01-01)
    let (year, month, day) = days_to_date(days);

    let iso = format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hours, minutes, seconds
    );

    let res = TimeResponse {
        status: "success".into(),
        timestamp_ms: ts_ms,
        iso_8601: iso,
    };

    let output = serde_json::to_vec(&res).unwrap_or_else(|_| alloc::vec::Vec::from(b"{\"status\":\"error\"}".as_slice()));
    let out_len = output.len() as u64;
    let out_ptr = output.as_ptr() as u64;
    core::mem::forget(output);

    (out_ptr << 32) | (out_len & 0xFFFFFFFF)
}

fn days_to_date(days: i64) -> (i64, u32, u32) {
    let mut d = days + 719468;
    let era = if d >= 0 { d } else { d - 146096 } / 146097;
    let doe = (d - era * 146097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if month <= 2 { y + 1 } else { y };
    (year, month, day)
}
