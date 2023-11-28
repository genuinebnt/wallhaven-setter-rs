mod search;

use futures::future::join_all;
use reqwest::Client;
use search::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls: Vec<String> = (1..5)
    .into_iter()
    .map(|i| format!("https://wallhaven.cc/api/v1/search?q=anime&categories=010&purity=111&atleast=1920x1080&sorting=views&order=desc&ratios=landscape&ai_art_filter=1&page={}&apikey=Uu2jVT0EJLBeoPf2S9S1ylGNJeXYWqRh", i)).collect();

    let client = Arc::new(Client::new());
    let mut tasks = Vec::new();

    for url in urls {
        let url = url.clone();
        let client = Arc::clone(&client);

        tasks.push(tokio::spawn(async move {
            let response = client.get(url).send().await.unwrap();
            match response.status() {
                reqwest::StatusCode::OK => {
                    match response.json::<Search>().await {
                        Ok(parsed) => {
                            process_links(parsed.data.into_iter().map(|x| x.path).collect())
                        }
                        Err(e) => println!("Error response didnt match shape we expected: {}", e),
                    };
                }

                reqwest::StatusCode::UNAUTHORIZED => println!("Authentication token is invalid"),
                _ => panic!("Unexpected error"),
            }
        }))
    }

    join_all(tasks).await;

    Ok(())
}

fn process_links(links: Vec<String>) {
    println!("{:?}", links);
}
