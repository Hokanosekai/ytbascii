use serde::de::DeserializeOwned;

trait ParseResponse {
  fn parse(json_str: &str) -> Result<Self, serde_json::Error> where Self: Sized;
}

struct SearchResponse {
  body: String,
}

struct VideoResponse {
  body: String,
}

trait Result {
  fn to_object<T>(json_str: &str) -> Result<T, String> where T: DeserializeOwned;
}

enum SearchResultItems {
  Video(VideoObject),
  Channel(ChannelObject),
  Playlist(PlaylistObject),
}

struct SearchResult {
  items: Vec<SearchResultItems>,
  next_page: Option<String>,
  prev_page: Option<String>,
  estimated_results: Option<i32>,
  query: String,
  params: SearchParams,
}

impl Result for SearchResult {
  fn parse<T>(json_str: &str) -> Result<T, String> where T: DeserializeOwned {
    serde_json::from_str(&self.body).map_err(|e| e.to_string())
  }
}