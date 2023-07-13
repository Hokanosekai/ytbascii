pub fn get_ytb_url(id: &str) -> String {
  format!("https://www.youtube.com/watch?v={}", id)
}

pub fn get_ytb_url_info(id: &str) -> String {
  format!("https://www.youtube.com/get_video_info?video_id={}", id)
}

pub fn get_ytb_api_info(id: &str, key: &str) -> String {
  format!("https://www.googleapis.com/youtube/v3/videos?id={}&part=snippet,statistics&key={}", id, key)
}

pub fn parse_number(value: serde_json::Value) -> u32 {
  if value.is_null() {
    0;
  }
  if value.is_number() {
    value.as_u64().unwrap() as u32
  } else if value.is_string() {
    value.as_str().unwrap().parse::<u32>().unwrap()
  } else {
    0
  }
}