use reqwest::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Vec<Daum>,
    pub meta: Meta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    pub id: String,
    pub url: String,
    #[serde(rename = "short_url")]
    pub short_url: String,
    pub views: i64,
    pub favorites: i64,
    pub source: String,
    pub purity: String,
    pub category: String,
    #[serde(rename = "dimension_x")]
    pub dimension_x: i64,
    #[serde(rename = "dimension_y")]
    pub dimension_y: i64,
    pub resolution: String,
    pub ratio: String,
    #[serde(rename = "file_size")]
    pub file_size: i64,
    #[serde(rename = "file_type")]
    pub file_type: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub colors: Vec<String>,
    pub path: String,
    pub thumbs: Thumbs,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbs {
    pub large: String,
    pub original: String,
    pub small: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    #[serde(rename = "current_page")]
    pub current_page: i64,
    #[serde(rename = "last_page")]
    pub last_page: i64,
    #[serde(rename = "per_page")]
    pub per_page: String,
    pub total: i64,
    pub query: Option<String>,
    pub seed: Option<String>,
}

#[tokio::main]
async fn main() {
    let url = "https://wallhaven.cc/api/v1/search?q=anime&categories=010&purity=111&atleast=1920x1080&sorting=views&order=desc&ratios=landscape&ai_art_filter=1&page=1&apikey=Uu2jVT0EJLBeoPf2S9S1ylGNJeXYWqRh";

    let client = Client::new();
    let response = client.get(url).send().await.unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<Root>().await {
                Ok(parsed) => process_links(parsed.data.into_iter().map(|x| x.path).collect()),
                Err(e) => println!("Error response didnt match shape we expected: {}", e),
            };
        }

        reqwest::StatusCode::UNAUTHORIZED => println!("Authentication token is invalid"),
        _ => panic!("Unexpected error"),
    }
}

fn process_links(links: Vec<String>) {
    println!("{:?}", links);
}
