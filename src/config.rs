use dotenvy::dotenv;
use std::env;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub wallhaven_url: String,
    pub query: String,
    pub api_key: String,
    pub categories: String,
    pub purity: String,
    pub sorting: String,
    pub order: String,
    pub atleast: String,
    pub ratios: String,
    pub ai_art_filter: String,
    pub downloads_path: String,
    pub pages_to_index: u32,
}

impl Config {
    pub fn new() -> Config {
        dotenv().ok();
        Config {
            wallhaven_url: env::var("SEARCH_URL").unwrap(),
            query: env::var("QUERY").unwrap(),
            api_key: env::var("API_KEY").unwrap(),
            categories: env::var("CATEGORIES").unwrap(),
            purity: env::var("PURITY").unwrap(),
            sorting: env::var("SORTING").unwrap(),
            order: env::var("ORDER").unwrap(),
            atleast: env::var("ATLEAST").unwrap(),
            ratios: env::var("RATIOS").unwrap(),
            ai_art_filter: env::var("AI_ART_FILTER").unwrap(),
            downloads_path: env::var("DOWNLOADS_PATH").unwrap(),
            pages_to_index: env::var("PAGES_TO_INDEX").unwrap().parse().unwrap(),
        }
    }
}
