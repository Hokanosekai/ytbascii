use futures::executor::block_on;

extern crate reqwest;

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

#[derive(Debug, PartialEq)]
pub enum QualityType {
  Tiny,
  Small,
  Medium,
  Large,
  HD720,
  HD1080,
  HD1440,
  HD2160,
  HD2880,
  HighRes,
}

#[derive(Debug)]
pub struct Stream {
  // The stream url
  pub url: String,
  // The stream quality
  pub quality: QualityType,
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
      quality: QualityType::from_string(data["quality"].as_str().unwrap().to_string()),
      quality_label: String::new(),
      bitrate: parse_number(data["bitrate"].clone()),
      mime_type: data["mimeType"].as_str().unwrap().to_string(),
      stream_type: StreamType::from_string(data["mimeType"].as_str().unwrap().to_string()),
      width: parse_number(data["width"].clone()),
      height: parse_number(data["height"].clone()),
      extension: data["mimeType"].to_string().split("/").collect::<Vec<&str>>()[1].split(";").collect::<Vec<&str>>()[0].to_string(),
      fps: 0,
      duration: parse_number(data["approxDurationMs"].clone()),
      content_length: parse_number(data["contentLength"].clone()),
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
      quality: self.quality.clone(),
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

    pub fn get_best_stream(&self, quality: QualityType) -> Stream {
      let mut best_stream = self.streams[0].clone();

      self.streams.iter().for_each(|stream| {
          if stream.quality == quality && stream.extension == "mp4" {
              best_stream = stream.clone();
          }
      });

      best_stream
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

impl QualityType {
  pub fn from_string(quality: String) -> QualityType {
    match quality.as_str() {
      "tiny" => QualityType::Tiny,
      "small" => QualityType::Small,
      "medium" => QualityType::Medium,
      "large" => QualityType::Large,
      "hd720" => QualityType::HD720,
      "hd1080" => QualityType::HD1080,
      "hd1440" => QualityType::HD1440,
      "hd2160" => QualityType::HD2160,
      "hd2880" => QualityType::HD2880,
      "highres" => QualityType::HighRes,
      _ => QualityType::Medium,
    }
  }

  pub fn to_string(&self) -> String {
    match self {
      QualityType::Tiny => "tiny",
      QualityType::Small => "small",
      QualityType::Medium => "medium",
      QualityType::Large => "large",
      QualityType::HD720 => "hd720",
      QualityType::HD1080 => "hd1080",
      QualityType::HD1440 => "hd1440",
      QualityType::HD2160 => "hd2160",
      QualityType::HD2880 => "hd2880",
      QualityType::HighRes => "highres",
    }.to_string()
  }

  pub fn clone(&self) -> QualityType {
    match self {
      QualityType::Tiny => QualityType::Tiny,
      QualityType::Small => QualityType::Small,
      QualityType::Medium => QualityType::Medium,
      QualityType::Large => QualityType::Large,
      QualityType::HD720 => QualityType::HD720,
      QualityType::HD1080 => QualityType::HD1080,
      QualityType::HD1440 => QualityType::HD1440,
      QualityType::HD2160 => QualityType::HD2160,
      QualityType::HD2880 => QualityType::HD2880,
      QualityType::HighRes => QualityType::HighRes,
    }
  }
}