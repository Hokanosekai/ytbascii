
use youtube::models::video::Video;
use logger::{Level, Logger};
use ffmpeg_next::{
    format::{input, Pixel},
    media::Type,
    software::scaling::{Context, Flags},
    frame::Video as Frame,
};
use std::io::{Write, stdout};
pub mod utils;


static mut LOGGER: Option<Logger> = None;

pub fn init_logger(level: Level) {
    unsafe {
        LOGGER = Some(Logger::new(level, "core".to_string()));
        LOGGER.as_ref().unwrap().info("Logger initialized".to_string());
    }
}

pub fn get_logger() -> &'static Logger {
    unsafe {
        LOGGER.as_ref().unwrap()
    }
}

pub fn init_loggers() {
    init_logger(Level::Info);
    youtube::init_logger(Level::Info);
}

fn save_frame(frame: &Frame, index: usize, output_path: &str) -> std::io::Result<()> {
    std::fs::create_dir_all(output_path)?;
    let mut file = std::fs::File::create(format!("{}/frame{}.jpg", output_path, index))?;
    file.write_all(format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes())?;
    file.write_all(frame.data(0))
}

fn save_ascii_art(frame: &Frame, index: usize, output_path: &str) -> std::io::Result<()> {
    
    let (cols, rows) = utils::get_shell_dim();

    std::fs::create_dir_all(output_path)?;
    let mut file = std::fs::File::create(format!("{}/frame{}.txt", output_path, index))?;
    let mut ascii_art = String::new();
    for y in 0..frame.height() {
        for x in 0..frame.width() {
            let pixel = frame.data(0)[(y * 0 + x * 3) as usize];
            let character = match pixel {
                0..=63 => ' ',
                64..=127 => '.',
                128..=191 => '*',
                192..=255 => '#',
            };
            ascii_art.push(character);
        }
        ascii_art.push('\n');
    }

    stdout().write_all(ascii_art.as_bytes())?;
    file.write_all(ascii_art.as_bytes())
}

fn extract_frames(input_path: &str, output_path: &str) {
    crate::get_logger().info(format!("Extracting frames from {}", input_path));
    ffmpeg_next::init().unwrap();

    match input(&input_path) {
        Ok(mut ictx) => {
            let input = ictx
                .streams()
                .best(Type::Video)
                .ok_or(ffmpeg_next::Error::StreamNotFound)
                .unwrap();
            let video_stream_index = input.index();

            let context_decoder =
                ffmpeg_next::codec::context::Context::from_parameters(input.parameters()).unwrap();
            let mut decoder = context_decoder.decoder().video().unwrap();

            let mut scaler = Context::get(
                decoder.format(),
                decoder.width(),
                decoder.height(),
                Pixel::RGB24,
                decoder.width(),
                decoder.height(),
                Flags::BILINEAR,
            ).unwrap();
            let mut frame_index: usize = 0;

            let mut receive_and_process_decoded_frames =
                |decoder: &mut ffmpeg_next::decoder::Video| -> Result<(), ffmpeg_next::Error> {
                    let mut decoded = Frame::empty();
                    while decoder.receive_frame(&mut decoded).is_ok() {
                        let mut rgb_frame = Frame::empty();
                        scaler.run(&decoded, &mut rgb_frame)?;
                        save_frame(&rgb_frame, frame_index, output_path).unwrap();
                        save_ascii_art(&rgb_frame, frame_index, output_path).unwrap();
                        frame_index += 1;
                    }
                    Ok(())
                };
            for (stream, packet) in ictx.packets() {
                if stream.index() == video_stream_index {
                    let _ = decoder.send_packet(&packet);
                    let _ = receive_and_process_decoded_frames(&mut decoder);
                }
            }
            let _ = decoder.send_eof();
            let _ = receive_and_process_decoded_frames(&mut decoder);
        }
        Err(e) => {
            crate::get_logger().error(format!("Error opening input file: {}", e));
        }
    }
}

#[tokio::main]
async fn main() -> () {
    // Init loggers
    init_loggers();

    // Get terminal dimensions
    let (cols, rows) = utils::get_shell_dim();

    let video = Video::new("BAyrperws4c").unwrap();


    // Download the best stream
    let file_path = video.download();

    let output_path = format!("data/{}/frames", video.id);

    // Extract frames
    extract_frames(file_path.as_str(), output_path.as_str());

}
