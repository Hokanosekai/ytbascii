pub mod client;
pub mod fetcher;

use lazy_static::lazy_static;
use std::sync::Mutex;

use logger::{create_logger, ModuleType, Logger};

// Define a lazy_static global instance of the CoreLogger
lazy_static! {
  static ref API_LOGGER: Mutex<Box<dyn Logger>> = Mutex::new(create_logger(ModuleType::API));
}

// Function to access the global CoreLogger instance
pub fn get_logger() -> std::sync::MutexGuard<'static, Box<dyn Logger>> {
  API_LOGGER.lock().expect("Failed to lock ApiLogger")
}

pub fn init_module() {
  get_logger().info("Initializing API module".to_string());
}


