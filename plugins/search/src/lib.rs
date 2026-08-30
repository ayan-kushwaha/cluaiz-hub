use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct SearchDoc {
    id: String,
    title: String,
    content: String,
}

#[derive(Deserialize)]
struct SearchRequest {
    query: String,
    #[serde(default)]
    documents: Option<Vec<SearchDoc>>,
    #[serde(default)]
    limit: Option<usize>,
}

#[derive(Serialize)]
struct SearchHit {
    id: String,
    title: String,
    snippet: String,
    score: f32,
}

#[derive(Serialize)]
struct SearchResponse {
    status: String,
    query: String,
    total_hits: usize,
    results: Vec<SearchHit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
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

    let response = match serde_json::from_slice::<SearchRequest>(input_bytes) {
        Ok(req) => perform_search(req),
        Err(_) => SearchResponse {
            status: "error".into(),
            query: String::new(),
            total_hits: 0,
            results: Vec::new(),
            error: Some("Invalid input JSON. Expected {\"query\": \"...\"}".into()),
        },
    };

    let output = serde_json::to_vec(&response)
        .unwrap_or_else(|_| Vec::from(b"{\"status\":\"error\",\"error\":\"Serialization failure\"}".as_slice()));

    let out_len = output.len() as u64;
    let out_ptr = output.as_ptr() as u64;
    core::mem::forget(output);

    (out_ptr << 32) | (out_len & 0xFFFFFFFF)
}

fn perform_search(req: SearchRequest) -> SearchResponse {
    let query_terms: Vec<String> = req
        .query
        .split_whitespace()
        .map(|s| s.to_ascii_lowercase())
        .filter(|s| !s.is_empty())
        .collect();

    if query_terms.is_empty() {
        return SearchResponse {
            status: "success".into(),
            query: req.query,
            total_hits: 0,
            results: Vec::new(),
            error: None,
        };
    }

    let limit = req.limit.unwrap_or(5);
    let mut hits = Vec::new();

    if let Some(docs) = req.documents {
        for doc in docs {
            let title_lower = doc.title.to_ascii_lowercase();
            let content_lower = doc.content.to_ascii_lowercase();

            let mut score = 0.0f32;
            for term in &query_terms {
                if title_lower.contains(term) {
                    score += 5.0;
                }
                // Count occurrences in content
                let mut count = 0;
                let mut start = 0;
                while let Some(pos) = content_lower[start..].find(term) {
                    count += 1;
                    start += pos + term.len();
                    if start >= content_lower.len() {
                        break;
                    }
                }
                score += count as f32;
            }

            if score > 0.0 {
                // Extract relevant snippet
                let snippet = extract_snippet(&doc.content, &query_terms[0]);
                hits.push(SearchHit {
                    id: doc.id,
                    title: doc.title,
                    snippet,
                    score,
                });
            }
        }
    }

    // Sort descending by score
    hits.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(core::cmp::Ordering::Equal));
    hits.truncate(limit);

    let total = hits.len();
    SearchResponse {
        status: "success".into(),
        query: req.query,
        total_hits: total,
        results: hits,
        error: None,
    }
}

fn extract_snippet(content: &str, first_term: &str) -> String {
    let content_lower = content.to_ascii_lowercase();
    if let Some(pos) = content_lower.find(first_term) {
        let start = if pos > 50 { pos - 50 } else { 0 };
        let end = if pos + 150 < content.len() { pos + 150 } else { content.len() };
        let slice = &content[start..end];
        let mut snippet = String::new();
        if start > 0 {
            snippet.push_str("...");
        }
        snippet.push_str(slice.trim());
        if end < content.len() {
            snippet.push_str("...");
        }
        snippet
    } else {
        let len = if content.len() > 150 { 150 } else { content.len() };
        let mut snippet = content[0..len].trim().to_string();
        if content.len() > 150 {
            snippet.push_str("...");
        }
        snippet
    }
}
