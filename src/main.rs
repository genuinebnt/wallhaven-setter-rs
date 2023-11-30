mod config;
mod downloader;
mod indexer;

use config::*;
use indexer::*;

use reqwest::Client;
use std::error::Error;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new();
    let client = Arc::new(Client::new());

    let indexer = Arc::new(Indexer::new(&config, Arc::clone(&client)));
    indexer.build_index(config.pages_to_index).await?;
    indexer.write_to_file("./index.txt")?;

    Ok(())
}
