use logger::{Level, Logger};

static mut LOGGER: Option<Logger> = None;

pub fn init_logger(level: Level) {
    unsafe {
        LOGGER = Some(Logger::new(level, "youtube".to_string()));
        LOGGER.as_ref().unwrap().info("Logger initialized".to_string());
    }
}

pub fn get_logger() -> &'static Logger {
    unsafe {
        LOGGER.as_ref().unwrap()
    }
}

pub mod models;
pub mod http;
pub mod utils;