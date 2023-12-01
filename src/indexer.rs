use crate::{config::Config, rate_limiter::RateLimiter};

use futures::future::join_all;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

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
    pub rate_limiter: Arc<Mutex<RateLimiter>>,
}

impl Indexer {
    pub fn new(config: &Config, client: Arc<Client>, requests_per_minute: u32) -> Indexer {
        let url = Self::build_url(config);
        let links = Arc::new(Mutex::new(HashMap::new()));

        let rate_limiter = Arc::new(Mutex::new(RateLimiter::new(requests_per_minute)));

        Indexer {
            url,
            links,
            client,
            rate_limiter,
        }
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
        let tasks = (1..=page_count)
            .map(|page| {
                let url = format!("{}&page={}", self.url, page);
                Self::process_url(
                    url,
                    Arc::clone(&self.client),
                    Arc::clone(&self.links),
                    Arc::clone(&self.rate_limiter),
                )
            })
            .collect::<Vec<_>>();

        join_all(tasks).await;

        Ok(())
    }

    async fn process_url(
        url: String,
        client: Arc<Client>,
        links: Arc<Mutex<HashMap<String, String>>>,
        rate_limiter: Arc<Mutex<RateLimiter>>,
    ) {
        let mut rate_limiter = rate_limiter.lock().await;
        if rate_limiter.counter <= 0 {
            eprintln!("Rate limit hit!");
            sleep(Duration::from_secs(
                60 / rate_limiter.requests_per_minute as u64,
            ))
            .await;

            rate_limiter.counter = rate_limiter.requests_per_minute;
        }
        drop(rate_limiter);

        match client.get(&url).send().await {
            Ok(response) => {
                let mut links = links.lock().await;

                if let Ok(parsed) = response.json::<Index>().await {
                    parsed.data.into_iter().for_each(|data| {
                        links.entry(data.id).or_insert(data.path);
                    });
                }
            }
            Err(err) => eprintln!("Error fetching {}: {}", url, err),
        }
    }

    pub async fn write_to_file(&self, path: &str) -> std::io::Result<()> {
        let data: String = self
            .links
            .lock()
            .await
            .iter()
            .map(|(key, value)| format!("wallhaven-{}:{}\n", key, value))
            .collect();

        std::fs::write(path, data)?;

        Ok(())
    }
}
