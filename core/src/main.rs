
use youtube::models::stream::StreamType;
use youtube::models::video::Video;

#[tokio::main]
async fn main() -> () {
    let video = Video::new("aPkSvjlzLx0").unwrap();
    let video_streams = video.streams.get_streams_by_type(StreamType::Video);
    video_streams.streams.iter().for_each(|stream| {
        println!("Stream : {:#?}", stream);
    });
    //println!("Video Streams : {:?}", video_streams);
}
