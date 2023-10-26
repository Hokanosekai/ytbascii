use std::io::Error;

use crate::get_logger;

use crate::fetcher::{SearchParams, VideoParams, SearchRequest, GetRequest, VideoRequest};

pub struct APIClient {
  api_url: String,
}

impl APIClient {
  pub fn new(url: String) -> Self {
    Self {
      api_url: url,
    }
  }
  pub fn search(&self, params: SearchParams) -> Result<(), Error> {
    get_logger().info(format!("Searching with params: {:?}", params));
    let request = SearchRequest::new(self.api_url.clone(), params);
    let response = request.execute();
    get_logger().info(format!("Response: {}", response.expect("Failed to execute request")));
    return Ok(());
  }
  pub fn get_video(&self, video_id: String, params: VideoParams) -> Result<(), Error> {
    get_logger().info(format!("Getting video with id: {}", video_id));
    let request = VideoRequest::new(self.api_url.clone(), video_id, params);
    let response = request.execute();
    get_logger().info(format!("Response: {}", response.expect("Failed to execute request")));
    return Ok(());
  }
}