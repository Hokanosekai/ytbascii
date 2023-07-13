extern crate reqwest;
extern crate tokio;
extern crate serde;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let url = "https://www.googleapis.com/youtube/v3/videos?id=aPkSvjlzLx0&part=snippet,statistics,fileDetails&key=AIzaSyDHTKjtUchUxUOzCtYW4V_h1zzcyd0P6c0";

    let response = reqwest::get(url)
        .await;

    let json = response.unwrap().text().await.unwrap();

    let data: serde_json::Value = serde_json::from_str(&json)?;

    println!("{}", data["videoDetails"]);
    println!("{:#?}", data);

    Ok(())
}