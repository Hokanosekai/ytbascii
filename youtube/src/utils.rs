pub fn get_ytb_url(id: &str) -> String {
  format!("https://www.youtube.com/watch?v={}", id)
}

pub fn get_ytb_url_info(id: &str) -> String {
  format!("https://www.youtube.com/get_video_info?video_id={}", id)
}

pub fn get_ytb_api_info(id: &str, key: &str) -> String {
  format!("https://www.googleapis.com/youtube/v3/videos?id={}&part=snippet,statistics&key={}", id, key)
}
