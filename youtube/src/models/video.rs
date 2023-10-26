use futures::executor::block_on;

use crate::http::{get_video_info, download_video_stream};
use crate::utils::parse_number;
use crate::models::error::Error;
use crate::models::stream::{StreamList, QualityType};
use crate::models::thumbnail::ThumbnailList;

#[derive(Debug)]
pub struct VideoInfo {
  // The video view count
  pub view_count: u32,
  // The video author (TODO: implement channel)
  pub author: String,
  // The video likes
  pub likes: u32,
  // The video comments count
  pub comment_count: u32,
  // The video category id
  pub category_id: u32,
  // The video upload date
  pub upload_date: String,
  // The video tags
  pub tags: Vec<String>,
}

#[derive(Debug)]
pub struct Video {
  // The video id
  pub id: String,
  // The video title
  pub title: String,
  // The video description
  pub description: String,
  // The available streams
  pub streams: StreamList,
  // The video info
  pub info: VideoInfo,
  // The video thumbnails
  pub thumbnails: ThumbnailList,
}

#[derive(Debug)]
pub struct VideoList {
  // The video list
  pub videos: Vec<Video>,
}

impl VideoInfo {
  pub fn from_json(json: serde_json::Value) -> VideoInfo {
    let data = json.clone();

    let snippet = data["snippet"].clone();
    let statistics = data["statistics"].clone();

    VideoInfo {
      tags: match snippet["tags"].as_array() {
        Some(tags) => tags.iter().map(|t| t.to_string()).collect(),
        None => vec![],
      },
      category_id: parse_number(snippet["categoryId"].clone()),
      comment_count: parse_number(statistics["commentCount"].clone()),
      likes: parse_number(statistics["likeCount"].clone()),
      view_count: parse_number(statistics["viewCount"].clone()),
      upload_date: snippet["publishedAt"].as_str().unwrap().to_string(),
      author: snippet["channelTitle"].as_str().unwrap().to_string(),
    }
  }
}

impl Video {
  pub fn new(id: &str) -> Result<Video, Error> {
    let video_info: serde_json::Value = match block_on(get_video_info(id)) {
      Ok(info) => info,
      Err(e) => return Err(Error::NetworkError(Box::new(e))),
    };

    if video_info["items"].as_array().unwrap().len() == 0 {
      return Err(Error::VideoNotFound);
    }

    Ok(Video::from_json(video_info["items"][0].clone()))

  }

  pub fn from_json(json: serde_json::Value) -> Video {
    let data = json.clone();

    Video { 
      id: data["id"].as_str().unwrap().to_string(), 
      title: data["snippet"]["title"].as_str().unwrap().to_string(), 
      description: data["snippet"]["description"].as_str().unwrap().to_string(),
      streams: StreamList::new(data["id"].as_str().unwrap()).unwrap(), 
      info: VideoInfo::from_json(data.clone()),
      thumbnails: ThumbnailList::from_json(data.clone()),
    }
  }

  pub fn download(&self) -> String {
    //crate::get_logger().info(format!("Downloading video {}", self.id));

    let mut stream = self.streams.get_best_stream(QualityType::Small);
    //crate::get_logger().info(format!("Downloading stream {}", stream.quality_label));

    let path = format!("data/{}/{:?}", self.id, stream.stream_type);
    std::fs::create_dir_all(path.clone())
      .expect("Failed to create videos directory");

    stream.set_file_path(format!("{}/{}.{}", path, stream.quality_label, stream.extension));

    stream = match block_on(download_video_stream(stream)) {
        Ok(r) => {
          //crate::get_logger().info(format!("Downloaded video {}", self.id));
          r
        },
        Err(e) => {
          //crate::get_logger().error(format!("Failed to download video {}", self.id));
          //crate::get_logger().error(format!("{:?}", e));
          panic!();
        }
    };

    stream.file_path.clone()
  }
}