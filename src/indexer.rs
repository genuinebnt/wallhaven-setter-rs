use crate::config::Config;

use futures::future::join_all;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::Write,
    sync::{Arc, Mutex},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    pub data: Vec<Data>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub id: String,
    pub url: String,
    pub path: String,
}

#[derive(Debug)]
pub struct Indexer {
    pub url: String,
    pub links: Arc<Mutex<HashMap<String, String>>>,
    pub client: Arc<Client>,
}

impl Indexer {
    pub fn new(config: &Config, client: Arc<Client>) -> Indexer {
        let url = Self::build_url(config);
        let links = Arc::new(Mutex::new(HashMap::new()));

        Indexer { url, links, client }
    }

    fn build_url(config: &Config) -> String {
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

    pub async fn build_index(&self, page_count: u32) -> Result<(), String> {
        let urls: Vec<String> = (1..=page_count)
            .map(|page| format!("{}&page={}", self.url, page))
            .collect();

        let mut tasks = Vec::new();

        for url in urls {
            let client = Arc::clone(&self.client);
            let links = Arc::clone(&self.links);

            tasks.push(tokio::spawn(async move {
                let response = client.get(url).send().await.unwrap();
                if let Some(parsed) = response.json::<Index>().await.ok() {
                    parsed.data.into_iter().for_each(|data| {
                        let mut links = links.lock().unwrap();
                        links.entry(data.id).or_insert(data.path);
                    })
                }
            }));
        }

        join_all(tasks).await;

        Ok(())
    }

    pub fn write_to_file(&self, path: &str) -> std::io::Result<()> {
        let links = self.links.lock().unwrap();
        let data: String = links
            .iter()
            .map(|(key, value)| format!("wallhaven_{}:{}\n", key, value))
            .collect();

        std::fs::write(path, data)?;

        Ok(())
    }
}
