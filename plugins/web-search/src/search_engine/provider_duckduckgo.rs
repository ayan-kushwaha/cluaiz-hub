use async_trait::async_trait;
use crate::search_engine::provider::{SearchProvider, SearchResult, get_favicon_url};
use reqwest::Client;

pub struct DuckDuckGoProvider;

#[async_trait]
impl SearchProvider for DuckDuckGoProvider {
    fn name(&self) -> &'static str {
        "DuckDuckGo (Free)"
    }

    async fn query(&self, query: &str) -> Result<Vec<SearchResult>, String> {
        let client = Client::builder()
            .user_agent("Cluaiz-Search/1.0 (https://github.com/cluaiz)")
            .build()
            .map_err(|e| format!("Failed to build client: {}", e))?;

        let url = format!("https://en.wikipedia.org/w/api.php?action=opensearch&search={}&limit=3&format=json", urlencoding::encode(query));
        let res = client.get(&url).send().await.map_err(|e| e.to_string())?;
        
        let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        
        let mut results = Vec::new();
        
        if let Some(titles) = json.get(1).and_then(|v| v.as_array()) {
            if let Some(urls) = json.get(3).and_then(|v| v.as_array()) {
                for (i, title_val) in titles.iter().enumerate() {
                    if let (Some(title), Some(url_val)) = (title_val.as_str(), urls.get(i)) {
                        if let Some(url) = url_val.as_str() {
                            results.push(SearchResult {
                                title: title.to_string(),
                                favicon: get_favicon_url(url),
                                url: url.to_string(),
                                snippet: Some(format!("Wikipedia article for {}", title)),
                                raw_content: None,
                            });
                        }
                    }
                }
            }
        }

        Ok(results)
    }
}
