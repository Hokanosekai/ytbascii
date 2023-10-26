extern crate reqwest;
extern crate serde;

use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::json;

use std::io::Write;
use curl::easy::Easy;

use crate::models::stream::Stream;

pub async fn get_video_info(video_id: &str) -> Result<serde_json::Value> {
		let url = format!("https://www.googleapis.com/youtube/v3/videos?id={}&part=snippet,statistics&key=AIzaSyDHTKjtUchUxUOzCtYW4V_h1zzcyd0P6c0",
				video_id);

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

pub async fn download_video_stream(stream: Stream) -> Result<Stream> {
	  let mut easy = Easy::new();
		easy.url(&stream.url).unwrap();

		//crate::get_logger().info(format!("Downloading video stream to {}", stream.file_path.clone()));
		//crate::get_logger().info(format!("Downloading {} bytes", stream.content_length));

		let mut file = std::fs::File::create(stream.file_path.clone())
				.expect("Failed to create file");

		let mut total_bytes = 0;
	
		easy.write_function(move |data| {
			file.write_all(data).unwrap();
			total_bytes += data.len();
			// Print a progress bar
			print!("\r[");
			for i in 0..50 {
				if i < (total_bytes as f32 / stream.content_length as f32 * 50.0) as usize {
					print!("=");
				} else {
					print!(" ");
				}
			}
			print!("] {}%", (total_bytes as f32 / stream.content_length as f32 * 100.0) as usize);
			std::io::stdout().flush().unwrap();
			Ok(data.len())
		}).unwrap();

		// Catch any errors
		match easy.perform() {
			Ok(_) => {
				std::io::stdout().flush().unwrap();
				println!();
			},
			Err(e) => {
				std::io::stdout().flush().unwrap();
				println!();
				//crate::get_logger().error(format!("Failed to download video stream to {}", stream.file_path.clone()));
				//crate::get_logger().error(format!("{:?}", e));
			}
		}

		Ok(stream)
}