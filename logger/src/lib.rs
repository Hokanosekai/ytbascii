
pub enum Level {
    Success,
    Info,
    Warn,
    Debug,
    Error,
}

pub struct Logger {
    pub level: Level,
    pub name: String,
}

impl Logger {
    pub fn new(level: Level, name: String) -> Logger {
        Logger { level, name }
    }

    pub fn log(&self, level: Level, message: String) {
        if self.level.compare(level.clone()) {
            println!("{}: [{}] {}", self.name, level.to_string(), message); 
        }
    }

    pub fn success(&self, message: String) {
        self.log(Level::Success, message);
    }

    pub fn info(&self, message: String) {
        self.log(Level::Info, message);
    }

    pub fn warn(&self, message: String) {
        self.log(Level::Warn, message);
    }

    pub fn debug(&self, message: String) {
        self.log(Level::Debug, message);
    }

    pub fn error(&self, message: String) {
        self.log(Level::Error, message);
    }
}

impl Level {
    pub fn from_u8(level: u8) -> Level {
        match level {
            0 => Level::Success,
            1 => Level::Info,
            2 => Level::Warn,
            3 => Level::Debug,
            4 => Level::Error,
            _ => Level::Info,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Level::Success => "Success".to_string(),
            Level::Info => "Info".to_string(),
            Level::Warn => "Warn".to_string(),
            Level::Debug => "Debug".to_string(),
            Level::Error => "Error".to_string(),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Level::Success => 0,
            Level::Info => 1,
            Level::Warn => 2,
            Level::Debug => 3,
            Level::Error => 4,
        }
    }

    pub fn compare(&self, level: Level) -> bool {
        self.to_u8() <= level.to_u8()
    }

    pub fn clone(&self) -> Level {
        Level::from_u8(self.to_u8())
    }
}

