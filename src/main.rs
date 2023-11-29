mod config;
mod downloader;
mod indexer;

use config::*;
use indexer::*;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new();
    let mut indexer = Indexer::new(config);
    indexer.build_index().await?;
    Ok(())
}
