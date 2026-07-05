use crate::search_engine::provider::SearchProvider;
use crate::search_engine::provider_tavily::TavilyProvider;
use crate::search_engine::provider_serpapi::SerpApiProvider;
use crate::search_engine::provider_duckduckgo::DuckDuckGoProvider;

/// Handles routing to the correct search provider based on the type
pub struct Rotator;

impl Rotator {
    pub fn get_provider(api_type: &str, api_key: &str) -> Box<dyn SearchProvider> {
        match api_type.to_lowercase().as_str() {
            "tavily" => Box::new(TavilyProvider { api_key: api_key.to_string() }),
            "serpapi" => Box::new(SerpApiProvider { api_key: api_key.to_string() }),
            "duckduckgo" => Box::new(DuckDuckGoProvider),
            // Fallback to DuckDuckGo (Free)
            _ => Box::new(DuckDuckGoProvider),
        }
    }
}
