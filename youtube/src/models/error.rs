#[derive(Debug)]
pub enum Error {
  VideoNotFound,
  NetworkError(Box<dyn std::error::Error>),
}