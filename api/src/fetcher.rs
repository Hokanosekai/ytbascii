extern crate reqwest;

use futures::executor::block_on;

use crate::get_logger;

// API endpoints
static SEARCH_ENDPOINT: &str = "/api/v1/search";
static VIDEO_ENDPOINT: &str = "/api/v1/videos/{}";

#[derive(Debug)]
pub struct SearchParams {
  pub q: String, // Query
  pub page: Option<u32>,
  pub sort_by: Option<String>, // relevance, rating, upload_date, view_count
  pub date: Option<String>, // hour, today, week, month, year
  pub duration: Option<String>, // short, long, medium
  pub ctype: Option<String>, // video, channel, playlist, all
  pub features: Option<Vec<String>>, // hd, subtitles, creative_commons, 3d, live, purchased, 4k, 360, location, hdr, vr180, (e.g. "hd,subtitles,creative_commons")
  pub region: Option<String>, // Default: US
}

pub struct VideoParams {
  pub region: Option<String>, // Default: US
}

pub enum ParamsType {
  Search(SearchParams),
  Video(VideoParams),
}

trait Params {
  fn build(&self) -> String;
}

impl Params for SearchParams {
  fn build(&self) -> String {
    let mut params = vec![];
    params.push(format!("q={}", self.q));
    if let Some(page) = self.page {
      params.push(format!("page={}", page));
    }
    if let Some(sort_by) = &self.sort_by {
      params.push(format!("sort_by={}", sort_by));
    }
    if let Some(date) = &self.date {
      params.push(format!("date={}", date));
    }
    if let Some(duration) = &self.duration {
      params.push(format!("duration={}", duration));
    }
    if let Some(ctype) = &self.ctype {
      params.push(format!("ctype={}", ctype));
    }
    if let Some(features) = &self.features {
      params.push(format!("features={}", features.join(",")));
    }
    if let Some(region) = &self.region {
      params.push(format!("region={}", region));
    }
    params.join("&")
  }
}

impl Params for VideoParams {
  fn build(&self) -> String {
    let mut params = vec![];
    if let Some(region) = &self.region {
      params.push(format!("region={}", region));
    }
    params.join("&")
  }
}

pub trait GetRequest {
  fn build_with_params(&self) -> String;
  fn execute(&self) -> Result<String, String>;
}

pub struct SearchRequest {
  url: String,
  params: ParamsType,
}

pub struct VideoRequest {
  url: String,
  video_id: String,
  params: ParamsType,
}

impl SearchRequest {
  pub fn new(api_url: String, params: SearchParams) -> Self {
    Self {
      url: format!("{}{}", api_url, SEARCH_ENDPOINT),
      params: ParamsType::Search(params),
    }
  }
}

impl VideoRequest {
  pub fn new(api_url: String, video_id: String, params: VideoParams) -> Self {
    Self {
      url: format!("{}{}", api_url, VIDEO_ENDPOINT),
      video_id,
      params: ParamsType::Video(params),
    }
  }
}

impl GetRequest for SearchRequest {
  fn build_with_params(&self) -> String {
    match &self.params {
      ParamsType::Search(params) => {
        let mut url = self.url.clone();
        let params = params.build();
        if !params.is_empty() {
          url.push_str("?");
          url.push_str(&params);
        }
        url
      },
      _ => panic!("Invalid params type"),
    }
  }
  fn execute(&self) -> Result<String, String> {
    let url = self.build_with_params();
    get_logger().debug(format!("Sending request to {}", url));
    let client = reqwest::Client::new();

    let response = block_on(client.get(url).send()).unwrap();

    let body = block_on(response.text()).map_err(|e| e.to_string())?;
    Ok(body)
  }
}

impl GetRequest for VideoRequest {
  fn build_with_params(&self) -> String {
    match &self.params {
      ParamsType::Video(params) => {
        let mut url = self.url.clone();
        url = url.replace("{}", &self.video_id);
        let params = params.build();
        if !params.is_empty() {
          url.push_str("?");
          url.push_str(&params);
        }
        url
      },
      _ => panic!("Invalid params type"),
    }
  }
  fn execute(&self) -> Result<String, String> {
    let url = self.build_with_params();
    get_logger().debug(format!("Sending request to {}", url));
    let client = reqwest::Client::new();

    let response = block_on(client.get(url).send()).unwrap();

    let body = block_on(response.text()).unwrap();
    Ok(body)
  }
}