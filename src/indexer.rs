use crate::config::Config;

use std::collections::HashMap;

pub struct Indexer {
    pub url: String,
    pub links: HashMap<String, String>,
}

impl Indexer {
    pub fn new(config: Config) -> Indexer {
        let url = Self::build_url(config);
        let links = HashMap::new();

        Indexer { url, links }
    }

    fn build_url(config: Config) -> String {
        format!(
            "{}q={}&categories={}&purity={}&sortings={}&order={}&atleast={}&ratios={}&ai_art_filter={}",
            config.wallhaven_url,
            config.query,
            config.categories,
            config.purity,
            config.sorting,
            config.order,
            config.atleast,
            config.ratios,
            config.ai_art_filter,
        )
    }

    pub async fn build_index(&mut self) -> Result<(), String> {
        todo!()
    }
}
