

#[derive(Debug)]
pub struct Stream {
  // The stream url
  url: String,
  // The stream quality
  quality: String,
  // The stream format
  extension: String,
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
  // The available audio streams
  pub audio_streams: StreamList,
  // The available video streams
  pub video_streams: StreamList,
}

#[derive(Debug)]
pub struct VideoInfo {
  // The video rating (0 to 5)
  pub rating: String,
  // The video view count
  pub view_count: u32,
  // The video author (TODO: implement channel)
  pub author: String,
  // The video length in seconds
  pub length: u32,
  // The video thumbnails
  pub thumbnail: VideoThumbnail,
  // The video likes
  pub likes: u32,
  // The video dislikes
  pub dislikes: u32,
  // The video comments count
  pub comment_count: u32,
  // The video category id
  pub category_id: u32,
  // The video upload date
  pub upload_date: String,
}

#[derive(Debug)]
pub struct VideoThumbnail {
  // The default thumbnail
  pub default: String,
  // The medium thumbnail
  pub medium: String,
  // The high thumbnail
  pub high: String,
  // The standard thumbnail
  pub standard: String,
  // The maxres thumbnail
  pub maxres: String,
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

impl Video {
  /*pub fn new(id: &str) -> Result<Video, Error> {
    let url_info = utils::get_ytb_url_info(id);
    let api_info = utils::get_ytb_api_info(id, "AIzaSyD-9tSrke72PouQMnMXwZ-VJ5jm0g8jZcg");

    // Get the video info
    /*let mut url_response = match utils::get_request(&url_info) {
      Ok(response) => response,
      Err(err) => return Err(Error::NetworkError(Box::new(err))),
    };
    let mut contents = String::new();
    url_response.read_to_string(&mut contents).unwrap();
    */

  }*/
}