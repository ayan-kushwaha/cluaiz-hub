use base64::Engine;
use serde::{Deserialize, Serialize};
use sha2::Digest;

#[derive(Deserialize)]
struct TextRequest {
    action: String,
    input: String,
    #[serde(default)]
    pattern: Option<String>,
    #[serde(default)]
    target: Option<String>,
}

#[derive(Serialize)]
struct TextResponse {
    status: String,
    action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matches: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diff_summary: Option<DiffSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stats: Option<TextStats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Serialize)]
struct TextStats {
    chars: usize,
    words: usize,
    lines: usize,
    bytes: usize,
}

#[derive(Serialize)]
struct DiffSummary {
    added_lines: usize,
    removed_lines: usize,
    common_lines: usize,
    diff_patch: String,
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

    let response = match serde_json::from_slice::<TextRequest>(input_bytes) {
        Ok(req) => process_text_request(req),
        Err(_) => TextResponse {
            status: "error".into(),
            action: "unknown".into(),
            result: None,
            matches: None,
            diff_summary: None,
            stats: None,
            error: Some("Invalid input JSON. Expected {\"action\": \"...\", \"input\": \"...\"}".into()),
        },
    };

    let output = serde_json::to_vec(&response)
        .unwrap_or_else(|_| Vec::from(b"{\"status\":\"error\",\"error\":\"Serialization failure\"}".as_slice()));

    let out_len = output.len() as u64;
    let out_ptr = output.as_ptr() as u64;
    core::mem::forget(output);

    (out_ptr << 32) | (out_len & 0xFFFFFFFF)
}

fn process_text_request(req: TextRequest) -> TextResponse {
    let action = req.action.to_ascii_lowercase();
    match action.as_str() {
        "hash" | "sha256" => {
            let mut hasher = sha2::Sha256::new();
            hasher.update(req.input.as_bytes());
            let hash_bytes = hasher.finalize();
            let mut hex_str = String::with_capacity(64);
            for b in hash_bytes {
                let _ = write_hex_byte(&mut hex_str, b);
            }
            TextResponse {
                status: "success".into(),
                action,
                result: Some(hex_str),
                matches: None,
                diff_summary: None,
                stats: None,
                error: None,
            }
        }
        "base64_encode" | "encode" => {
            let encoded = base64::engine::general_purpose::STANDARD.encode(req.input.as_bytes());
            TextResponse {
                status: "success".into(),
                action,
                result: Some(encoded),
                matches: None,
                diff_summary: None,
                stats: None,
                error: None,
            }
        }
        "base64_decode" | "decode" => {
            match base64::engine::general_purpose::STANDARD.decode(req.input.as_bytes()) {
                Ok(bytes) => match String::from_utf8(bytes) {
                    Ok(s) => TextResponse {
                        status: "success".into(),
                        action,
                        result: Some(s),
                        matches: None,
                        diff_summary: None,
                        stats: None,
                        error: None,
                    },
                    Err(_) => TextResponse {
                        status: "error".into(),
                        action,
                        result: None,
                        matches: None,
                        diff_summary: None,
                        stats: None,
                        error: Some("Decoded bytes are not valid UTF-8".into()),
                    },
                },
                Err(e) => TextResponse {
                    status: "error".into(),
                    action,
                    result: None,
                    matches: None,
                    diff_summary: None,
                    stats: None,
                    error: Some(format!("Base64 decoding failed: {:?}", e)),
                },
            }
        }
        "stats" | "count" => {
            let chars = req.input.chars().count();
            let bytes = req.input.len();
            let lines = if req.input.is_empty() { 0 } else { req.input.lines().count() };
            let words = req.input.split_whitespace().count();

            TextResponse {
                status: "success".into(),
                action,
                result: None,
                matches: None,
                diff_summary: None,
                stats: Some(TextStats { chars, words, lines, bytes }),
                error: None,
            }
        }
        "diff" => {
            let target = req.target.unwrap_or_default();
            let src_lines: Vec<&str> = req.input.lines().collect();
            let tgt_lines: Vec<&str> = target.lines().collect();

            let mut patch = String::new();
            let mut added = 0;
            let mut removed = 0;
            let mut common = 0;

            let max_len = if src_lines.len() > tgt_lines.len() { src_lines.len() } else { tgt_lines.len() };
            for i in 0..max_len {
                match (src_lines.get(i), tgt_lines.get(i)) {
                    (Some(s), Some(t)) if s == t => {
                        common += 1;
                        patch.push_str(" ");
                        patch.push_str(s);
                        patch.push('\n');
                    }
                    (Some(s), Some(t)) => {
                        removed += 1;
                        added += 1;
                        patch.push_str("-");
                        patch.push_str(s);
                        patch.push_str("\n+");
                        patch.push_str(t);
                        patch.push('\n');
                    }
                    (Some(s), None) => {
                        removed += 1;
                        patch.push_str("-");
                        patch.push_str(s);
                        patch.push('\n');
                    }
                    (None, Some(t)) => {
                        added += 1;
                        patch.push_str("+");
                        patch.push_str(t);
                        patch.push('\n');
                    }
                    (None, None) => {}
                }
            }

            TextResponse {
                status: "success".into(),
                action,
                result: None,
                matches: None,
                diff_summary: Some(DiffSummary {
                    added_lines: added,
                    removed_lines: removed,
                    common_lines: common,
                    diff_patch: patch,
                }),
                stats: None,
                error: None,
            }
        }
        "extract" | "find" | "regex" => {
            let pattern = req.pattern.unwrap_or_default();
            if pattern.is_empty() {
                return TextResponse {
                    status: "error".into(),
                    action,
                    result: None,
                    matches: None,
                    diff_summary: None,
                    stats: None,
                    error: Some("Pattern parameter is required for regex/extract action".into()),
                };
            }

            // Lightweight deterministic substring / pattern finder
            let mut matches = Vec::new();
            let mut start = 0;
            while let Some(pos) = req.input[start..].find(&pattern) {
                let actual_idx = start + pos;
                matches.push(pattern.clone());
                start = actual_idx + pattern.len();
                if start >= req.input.len() {
                    break;
                }
            }

            TextResponse {
                status: "success".into(),
                action,
                result: Some(format!("Found {} occurrences", matches.len())),
                matches: Some(matches),
                diff_summary: None,
                stats: None,
                error: None,
            }
        }
        _ => TextResponse {
            status: "error".into(),
            action,
            result: None,
            matches: None,
            diff_summary: None,
            stats: None,
            error: Some(format!("Unsupported action '{}'. Supported: sha256, base64_encode, base64_decode, stats, diff, extract", req.action)),
        },
    }
}

fn write_hex_byte(s: &mut String, b: u8) {
    const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";
    s.push(HEX_CHARS[(b >> 4) as usize] as char);
    s.push(HEX_CHARS[(b & 0x0F) as usize] as char);
}
