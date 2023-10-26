pub mod colors;

use colors::{YELLOW, CYAN, GREEN, RESET, PURPLE, RED, ITALIC};

// Define a Level enum
pub enum LogLevel {
    Success,
    Info,
    Warn,
    Debug,
    Error,
}

// Implement Level
impl LogLevel {
    pub fn to_string(&self) -> String {
        match self {
            LogLevel::Success => format!("{}{}{}", GREEN, "Success".to_string(), RESET),
            LogLevel::Info => format!("{}{}{}", CYAN, "Info".to_string(), RESET),
            LogLevel::Warn => format!("{}{}{}", YELLOW, "Warn".to_string(), RESET),
            LogLevel::Debug => format!("{}{}{}", PURPLE, "Debug".to_string(), RESET),
            LogLevel::Error => format!("{}{}{}", RED, "Error".to_string(), RESET),
        }
    }
}

// Define a Logger trait
pub trait Logger: Send + Sync {
    fn log(&self, message: &str, level: LogLevel);
    fn success(&self, message: String) {
        self.log(&message, LogLevel::Success);
    }
    fn info(&self, message: String) {
        self.log(&message, LogLevel::Info);
    }
    fn warn(&self, message: String) {
        self.log(&message, LogLevel::Warn);
    }
    fn debug(&self, message: String) {
        self.log(&message, LogLevel::Debug);
    }
    fn error(&self, message: String) {
        self.log(&message, LogLevel::Error);
    }
}

// Implement Logger for API module
pub struct APILogger;

impl Logger for APILogger {
    fn log(&self, message: &str, level: LogLevel) {
        println!("[{}API{}] [{}] {}", ITALIC, RESET, level.to_string(), message);
    }
}

// Implement Logger for UI module
pub struct UiLogger;

impl Logger for UiLogger {
    fn log(&self, message: &str, level: LogLevel) {
        println!("[{}UI{}] [{}] {}", ITALIC, RESET, level.to_string(), message);
    }
}

// Implement Logger for Core module
pub struct CoreLogger;

impl Logger for CoreLogger {
    fn log(&self, message: &str, level: LogLevel) {
        println!("[{}Core{}] [{}] {}", ITALIC, RESET, level.to_string(), message);
    }
}

// Enum to represent different modules
pub enum ModuleType {
    API,
    Ui,
    Core,
}

// Function to create a logger for a specific module
pub fn create_logger(module_type: ModuleType) -> Box<dyn Logger> {
    match module_type {
        ModuleType::API => Box::new(APILogger),
        ModuleType::Ui => Box::new(UiLogger),
        ModuleType::Core => Box::new(CoreLogger),
    }
}