use futures::executor::block_on;

use crate::{http::{get_video_info, get_video_streams}, utils::parse_number};

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

impl Thumbnail {
    pub fn from_json(json: serde_json::Value) -> Thumbnail {
      Thumbnail {
        url: json["url"].as_str().unwrap().to_string(),
        width: parse_number(json["width"].clone()),
        height: parse_number(json["height"].clone()),
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
      id: data["id"].as_str().unwrap().to_string(), 
      title: data["snippet"]["title"].as_str().unwrap().to_string(), 
      description: data["snippet"]["description"].as_str().unwrap().to_string(),
      streams: StreamList::new(data["id"].as_str().unwrap()).unwrap(), 
      info: VideoInfo::from_json(data.clone()),
      thumbnails: ThumbnailList::from_json(data.clone()),
    }
  }
}

impl Stream {
  pub fn from_json(json: serde_json::Value) -> Stream {
    let data = json.clone();

    let mut stream = Stream {
      url: data["url"].as_str().unwrap().to_string(),
      quality: data["quality"].as_str().unwrap().to_string(),
      quality_label: String::new(),
      bitrate: parse_number(data["bitrate"].clone()),
      mime_type: data["mimeType"].as_str().unwrap().to_string(),
      stream_type: StreamType::from_string(data["mimeType"].as_str().unwrap().to_string()),
      width: parse_number(data["width"].clone()),
      height: parse_number(data["height"].clone()),
      extension: data["mimeType"].to_string().split("/").collect::<Vec<&str>>()[1].split(";").collect::<Vec<&str>>()[0].to_string(),
      fps: 0,
      duration: parse_number(data["approxDurationMs"].clone()),
      content_length: 0,
      file_path: String::new(),
    };

    if stream.stream_type == StreamType::Video {
      stream.fps = parse_number(data["fps"].clone());
      stream.quality_label = data["qualityLabel"].as_str().unwrap().to_string();
    }

    stream
  }

  pub fn set_file_path(&mut self, file_path: String) {
    self.file_path = file_path.to_string();
  }
}

impl StreamList {
    pub fn new(video_id: &str) -> Result<StreamList, Error> {
      let video_streams = match block_on(get_video_streams(video_id)) {
        Ok(streams) => streams,
        Err(e) => return Err(Error::NetworkError(Box::new(e))),
      };

      let adaptive_formats = &video_streams["streamingData"]["adaptiveFormats"].as_array();
      let mut streams: Vec<Stream> = Vec::new();

      adaptive_formats.unwrap().iter().for_each(|format| {
        let stream = Stream::from_json(format.clone());
        streams.push(stream);
      });

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