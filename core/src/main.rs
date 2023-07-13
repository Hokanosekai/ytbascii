

#[tokio::main]
async fn main() -> () {
    let video = youtube::models::Video::new("aPkSvjlzLx0").unwrap();
    println!("Video : {:?}", video);
}