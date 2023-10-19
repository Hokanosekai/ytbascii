use crate::utils::parse_number;

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