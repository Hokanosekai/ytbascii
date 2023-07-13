use futures::executor::block_on;

use crate::http::{get_video_info, get_video_streams};

#[derive(Debug, PartialEq)]
pub enum StreamType {
  Audio,
  Video,
  AudioVideo,
  Unknown,
}

#[derive(Debug)]
pub struct Stream {
  // The stream url
  pub url: String,
  // The stream quality
  pub quality: String,
  // The stream quality label
  pub quality_label: String,
  // The stream bitrate
  pub bitrate: u32,
  // The stream type
  pub mime_type: String,
  // The stream type
  pub stream_type: StreamType,
  // The stream width
  pub width: u32,
  // The stream height
  pub height: u32,
  // The stream format
  pub extension: String,
  // The stream fps
  pub fps: u32,
  // The stream duration
  pub duration: u32,
  // The stream content length
  pub content_length: u32,
  // The stream file path
  pub file_path: String,
}

#[derive(Debug)]
pub struct StreamList {
  // The available streams
  streams: Vec<Stream>,
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
pub struct Thumbnail {
  // The thumbnail url
  pub url: String,
  // The thumbnail width
  pub width: u32,
  // The thumbnail height
  pub height: u32,
}

#[derive(Debug)]
pub struct ThumbnailList {
  // The available thumbnails
  pub thumbnails: Vec<Thumbnail>,
}

#[derive(Debug)]
pub struct VideoList {
  // The video list
  pub videos: Vec<Video>,
}

#[derive(Debug)]
pub enum Error {
  VideoNotFound,
  NetworkError(Box<dyn std::error::Error>),
}

impl VideoInfo {
  pub fn from_json(json: serde_json::Value) -> VideoInfo {
    let data = json.clone();

    let snippet = data["snippet"].clone();
    let statistics = data["statistics"].clone();

    let category_id: u32 = match snippet.get("categoryId") {
      Some(category_id) => category_id.to_string().parse::<u32>().unwrap(),
      None => 0,
    };
    println!("category_id: {}", category_id);

    VideoInfo {
      tags: snippet["tags"].as_array().unwrap().iter().map(|tag| tag.to_string()).collect(),
      category_id: snippet["categoryId"].as_u64().unwrap() as u32,
      comment_count: statistics["commentCount"].as_u64().unwrap() as u32,
      likes: statistics["likeCount"].as_u64().unwrap() as u32,
      view_count: statistics["viewCount"].as_u64().unwrap() as u32,
      upload_date: snippet["publishedAt"].to_string(),
      author: snippet["channelTitle"].to_string(),
    }
  }
}

impl Thumbnail {
    pub fn from_json(json: serde_json::Value) -> Thumbnail {
      Thumbnail {
        url: json["url"].to_string(),
        width: json["width"].as_u64().unwrap() as u32,
        height: json["height"].as_u64().unwrap() as u32,
      }
    }
}

impl ThumbnailList {
    pub fn from_json(json: serde_json::Value) -> ThumbnailList {
      let data = json["snippet"]["thumbnails"].clone();

      ThumbnailList {
        thumbnails: vec![
          Thumbnail::from_json(data["default"].clone()),
          Thumbnail::from_json(data["medium"].clone()),
          Thumbnail::from_json(data["high"].clone()),
          Thumbnail::from_json(data["standard"].clone()),
          Thumbnail::from_json(data["maxres"].clone()),
        ]
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
      id: data["id"].to_string(), 
      title: data["snippet"]["title"].to_string(), 
      description: data["snippet"]["description"].to_string(),
      streams: StreamList::new(data["id"].to_string()).unwrap(), 
      info: VideoInfo::from_json(data.clone()),
      thumbnails: ThumbnailList::from_json(data.clone()),
    }
  }
}

impl Stream {
  pub fn from_json(json: serde_json::Value) -> Stream {
    let data = json.clone();

    let mut stream = Stream {
      url: data["url"].to_string(),
      quality: data["quality"].to_string(),
      quality_label: data["qualityLabel"].to_string(),
      bitrate: data["bitrate"].as_u64().unwrap() as u32,
      mime_type: data["mimeType"].to_string(),
      stream_type: StreamType::from_string(data["mimeType"].to_string()),
      width: data["width"].as_u64().unwrap() as u32,
      height: data["height"].as_u64().unwrap() as u32,
      extension: String::new(),
      fps: 0,
      duration: data["approxDurationMs"].as_u64().unwrap() as u32,
      content_length: 0,
      file_path: String::new(),
    };

    if stream.stream_type == StreamType::Video {
      stream.extension = data["mimeType"].to_string().split("/").collect::<Vec<&str>>()[1].to_string();
      stream.fps = data["fps"].as_u64().unwrap() as u32;
    }

    if stream.stream_type == StreamType::Audio {
      stream.extension = data["mimeType"].to_string().split("/").collect::<Vec<&str>>()[1].to_string();
    }

    stream
  }

  pub fn set_file_path(&mut self, file_path: String) {
    self.file_path = file_path.to_string();
  }
}

impl StreamList {
    pub fn new(videoID: String) -> Result<StreamList, Error> {
      let video_streams = match block_on(get_video_streams(&videoID)) {
        Ok(streams) => streams,
        Err(e) => return Err(Error::NetworkError(Box::new(e))),
      };

      let adaptive_formats = &video_streams["streamingData"]["adaptiveFormats"];
      println!("{:?}", adaptive_formats);
      let mut streams: Vec<Stream> = Vec::new();

      /*for format in adaptive_formats {
        let mut stream = Stream::from_json(format.clone());
        streams.push(stream);
      }*/

      Ok(StreamList { streams })
    }
}

impl StreamType {
    pub fn from_string(mime_type: String) -> StreamType {
      if mime_type.contains("video") {
        return StreamType::Video;
      }

      if mime_type.contains("audio") {
        return StreamType::Audio;
      }

      StreamType::Unknown
    }
}