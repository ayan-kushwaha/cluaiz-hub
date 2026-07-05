use serde_json::Value;

pub struct DynamicConfig {
    pub search_api_key: String,
    pub search_api_type: String,
    pub think_mode: bool,
    pub think_mode_str: String,
    pub response_length: String,
}

pub async fn get_dynamic_config(payload: &Value) -> Result<DynamicConfig, String> {
    // Extract search_api_key and search_api_type from the Engine's injected payload
    let search_api_key = payload
        .get("search_api_key")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let search_api_type = payload
        .get("search_api_type")
        .and_then(|v| v.as_str())
        .unwrap_or("duckduckgo")
        .to_string();

    // Extract system_booster settings injected by the Engine via system_bindings
    let mut think_mode = false;
    let mut think_mode_str = "off".to_string();
    let mut response_length = "auto".to_string();

    if let Some(booster) = payload.get("system_booster") {
        think_mode_str = booster
            .get("think_mode")
            .and_then(|t| t.as_str())
            .or_else(|| {
                booster
                    .get("think_mode")
                    .and_then(|t| t.get("state"))
                    .and_then(|s| s.as_str())
            })
            .unwrap_or("auto")
            .to_string();

        think_mode = think_mode_str.to_lowercase() == "on";

        response_length = booster
            .get("response_length")
            .and_then(|r| r.as_str())
            .unwrap_or("auto")
            .to_string();
    }

    Ok(DynamicConfig {
        search_api_key,
        search_api_type,
        think_mode,
        think_mode_str,
        response_length,
    })
}
