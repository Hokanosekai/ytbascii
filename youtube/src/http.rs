extern crate reqwest;
extern crate serde;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::json;

use crate::models::stream::Stream;

pub async fn get_video_info(video_id: &str) -> Result<serde_json::Value> {
		let url = format!("https://www.googleapis.com/youtube/v3/videos?id={}&part=snippet,statistics&key=AIzaSyDHTKjtUchUxUOzCtYW4V_h1zzcyd0P6c0",
				video_id);
		println!("{}", url);

		let client = reqwest::Client::new();

		let response = client.get(url)
				.header("Content-Type", "application/json")
				.send()
				.await;

		let json = response.unwrap().text().await.unwrap();

		let data: serde_json::Value = serde_json::from_str(&json)?;

		Ok(data)
}

#[derive(Serialize, Deserialize, Debug)]
struct VideoStreamsPayload<'a> {
		context: serde_json::Value,
		video_id: &'a str,
}

pub async fn get_video_streams(video_id: &str) -> Result<serde_json::Value> {
		let json_data = json!({
			"context": {
				"client": {
					"hl": "en",
					"clientName": "WEB",
					"clientVersion": "2.20210721.00.00",
					"mainAppWebInfo": {
							"graftUrl": format!("/watch?v={}", video_id),
					}
				}
			},
			"video_id": video_id,
		});

		
		println!("{}", json_data.to_string());
		let url = "https://youtubei.googleapis.com/youtubei/v1/player?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8";

		let client = reqwest::Client::new();

		let response = client.post(url)
				.body(json_data.to_string())
				.send()
				.await;

		let json = response.unwrap().text().await.unwrap();

		let data: serde_json::Value = serde_json::from_str(&json)?;

		Ok(data)
}

pub async fn download_video_stream(video_id: &str, mut stream: Stream) -> Result<()> {
		let client = reqwest::Client::new();

		let response = client.get(stream.url.clone())
				.header("Content-Type", "application/json")
				.send()
				.await;

		let _path = format!("data/{}/{:?}", video_id, stream.stream_type);
		std::fs::create_dir_all("data/{}")
				.expect("Failed to create videos directory");

		stream.set_file_path(format!("data/{}/{:?}/{}", video_id, stream.stream_type, stream.quality_label));

		let mut file = match std::fs::File::open(stream.file_path.clone()) {
				Ok(file) => file,
				Err(_) => std::fs::File::create("video.mp4").unwrap()
		};
		let content = response.unwrap().bytes().await.unwrap();

		std::io::copy(&mut content.as_ref(), &mut file).unwrap();

		Ok(())
}