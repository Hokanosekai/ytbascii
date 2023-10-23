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
            LogLevel::Success => "Success".to_string(),
            LogLevel::Info => "Info".to_string(),
            LogLevel::Warn => "Warn".to_string(),
            LogLevel::Debug => "Debug".to_string(),
            LogLevel::Error => "Error".to_string(),
        }
    }
}

// Define a Logger trait
trait Logger {
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

// Implement Logger for Fetcher module
struct FetcherLogger;

impl Logger for FetcherLogger {
    fn log(&self, message: &str, level: LogLevel) {
        println!("[Fetcher] [{}] {}", level.to_string(), message);
    }
}

// Implement Logger for UI module
struct UiLogger;

impl Logger for UiLogger {
    fn log(&self, message: &str, level: LogLevel) {
        println!("[UI] [{}] {}", level.to_string(), message);
    }
}

// Implement Logger for Core module
struct CoreLogger;

impl Logger for CoreLogger {
    fn log(&self, message: &str, level: LogLevel) {
        println!("[Core] [{}] {}", level.to_string(), message);
    }
}

// Enum to represent different modules
pub enum ModuleType {
    Fetcher,
    Ui,
    Core,
}

// Function to create a logger for a specific module
pub fn create_logger(module_type: ModuleType) -> Box<dyn Logger> {
    match module_type {
        ModuleType::Fetcher => Box::new(FetcherLogger),
        ModuleType::Ui => Box::new(UiLogger),
        ModuleType::Core => Box::new(CoreLogger),
    }
}