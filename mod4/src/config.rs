#[derive(Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}
#[derive(Debug)]
pub enum LogOutput {
    Stdout,
    Stderr,
    File(String),
}
#[derive(Debug)]
pub struct Logging {
    pub enabled: bool,
    pub level: LogLevel,
    pub destination: LogOutput,
}

impl Logging {
    pub fn new() -> Self {
        Self {
            enabled: false,
            level: LogLevel::Info,
            destination: LogOutput::Stdout,
        }
    }
}

pub fn run() {
    let log_demo = Logging::new();
    println!("this is output {:?}", log_demo);
}
