use anyhow::Result;
use chrono::Utc;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct Payload {
    payload: Listens,
}

#[derive(Deserialize)]
struct Listens {
    listens: Vec<Value>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let base_url = String::from("https://api.listenbrainz.org");

    let home = std::env::var("HOME")?;
    let username = std::fs::read_to_string(format!("{}/.config/listenbrainz/username", home))?;
    let token = std::fs::read_to_string(format!("{}/.config/listenbrainz/token", home))?;
    let min_timestamp = Utc::now()
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp();

    let username = username.trim();
    let token = token.trim();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/1/user/{}/listens", base_url, username))
        .header("Authorization", format!("Token {}", token))
        .query(&[("min_ts", min_timestamp), ("count", 1000)])
        .send()
        .await?;

    if response.status() != StatusCode::OK {
        return Ok(());
    }

    let json = response.json::<Payload>().await?;

    println!("{}", json.payload.listens.len());

    Ok(())
}
