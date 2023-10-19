use futures::executor::block_on;

use crate::http::get_video_streams;
use crate::utils::parse_number;
use crate::models::error::Error;

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
  pub streams: Vec<Stream>,
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

  pub fn clone(&self) -> Stream {
    Stream {
      url: self.url.to_string(),
      quality: self.quality.to_string(),
      quality_label: self.quality_label.to_string(),
      bitrate: self.bitrate,
      mime_type: self.mime_type.to_string(),
      stream_type: self.stream_type.clone(),
      width: self.width,
      height: self.height,
      extension: self.extension.to_string(),
      fps: self.fps,
      duration: self.duration,
      content_length: self.content_length,
      file_path: self.file_path.to_string(),
    }
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

    pub fn get_streams_by_type(&self, stream_type: StreamType) -> StreamList {
      let mut streams: Vec<Stream> = Vec::new();

      self.streams.iter().for_each(|stream| {
          if stream.stream_type == stream_type {
              streams.push(stream.clone());
          }
      });

      StreamList { streams }
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

    pub fn to_string(&self) -> String {
      match self {
        StreamType::Video => "video",
        StreamType::Audio => "audio",
        StreamType::AudioVideo => "audiovideo",
        StreamType::Unknown => "unknown",
      }.to_string()
    }

    pub fn clone(&self) -> StreamType {
      match self {
        StreamType::Video => StreamType::Video,
        StreamType::Audio => StreamType::Audio,
        StreamType::AudioVideo => StreamType::AudioVideo,
        StreamType::Unknown => StreamType::Unknown,
      }
    }
}