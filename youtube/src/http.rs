extern crate reqwest;
extern crate serde;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::json;

use crate::models::Stream;

pub async fn get_video_info(videoID: &str) -> Result<serde_json::Value> {
		let url = format!("https://www.googleapis.com/youtube/v3/videos?id={}&part=snippet,statistics&key=AIzaSyDHTKjtUchUxUOzCtYW4V_h1zzcyd0P6c0",
				videoID);

		let client = reqwest::Client::new();

		let response = client.get(url)
				.header("Content-Type", "application/json")
				.send()
				.await;

		let json = response.unwrap().text().await.unwrap();

		let data: serde_json::Value = serde_json::from_str(&json)?;

		Ok(data)
}

pub async fn get_video_streams(videoID: &str) -> Result<serde_json::Value> {
		let jsonData = json!({
				"context": {
					"client": {
					 "hl": "en",
					 "clientName": "WEB",
					 "clientVersion": "2.20210721.00.00",
					 "mainAppWebInfo": {
							 "graftUrl": format!("/watch?v={}", videoID)
					 }
					}
				 },
				 "videoId": videoID
			 });
		let url = "https://youtubei.googleapis.com/youtubei/v1/player?key=AIzaSyB-9tSrke72PouQMnXJqjZxY5QZ6Z6qZ5o";

		let client = reqwest::Client::new();

		let response = client.post(url)
				.header("Content-Type", "application/json")
				.body(jsonData.to_string())
				.send()
				.await;

		let json = response.unwrap().text().await.unwrap();

		let data: serde_json::Value = serde_json::from_str(&json)?;

		Ok(data)
}

pub async fn download_video_stream(videoID: &str, mut stream: Stream) -> Result<()> {
		let client = reqwest::Client::new();

		let response = client.get(stream.url.clone())
				.header("Content-Type", "application/json")
				.send()
				.await;

		let path = format!("data/{}/{:?}", videoID, stream.stream_type);
		std::fs::create_dir_all("data/{}")
				.expect("Failed to create videos directory");

		stream.set_file_path(format!("data/{}/{:?}/{}", videoID, stream.stream_type, stream.quality_label));

		let mut file = match std::fs::File::open(stream.file_path.clone()) {
				Ok(file) => file,
				Err(_) => std::fs::File::create("video.mp4").unwrap()
		};
		let mut content = response.unwrap().bytes().await.unwrap();

		std::io::copy(&mut content.as_ref(), &mut file).unwrap();

		Ok(())
}