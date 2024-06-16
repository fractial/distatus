use std::{env, fs};
use std::time::Duration;
use dotenv::dotenv;
use reqwest::{Client, Response};
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use rand::seq::IndexedRandom;

const URL: &str = "https://discordapp.com/api/v8/users/@me/settings";
const FILE: &str = "status.json";

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Deserialize, Serialize)]
struct Status {
    custom_status: Option<CustomStatus>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CustomStatus {
    emoji_id: Option<u64>,
    emoji_name: Option<String>,
    text: String,
}

fn read_statuses(file: &str) -> Result<Vec<CustomStatus>> {
    let json = fs::read_to_string(file)?;
    let statuses: Vec<CustomStatus> = serde_json::from_str(&json)?;

    Ok(statuses)
}

async fn patch_status(url: &str, auth: &str, status: &Status) -> Result<Response> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(auth)?);
    let response = client.patch(url).headers(headers).json(&status).send().await?;
    let status = response.status();

    if !status.is_success() {
        return Err(format!("request failed with status: `{}`", status).into());
    };

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let auth = env::var("TOKEN")?;
    let statuses = read_statuses(FILE)?;

    loop {
        let random_status = statuses.choose(&mut rand::thread_rng()).map(|status| CustomStatus {
            emoji_id: status.emoji_id.clone(),
            emoji_name: status.emoji_name.clone(),
            text: status.text.clone(),
        }).unwrap_or_else(|| {
            println!("No statuses found.");
            std::process::exit(1);
        });

        let status = Status {
            custom_status: Some(random_status),
        };

        if let Err(err) = patch_status(URL, &auth, &status).await {
            eprintln!("Error {}", err);
        }

        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
