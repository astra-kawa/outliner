use crate::services::errors::ServiceError;

pub trait LoggingService {
    fn write_log(&self, message: String) -> Result<(), ServiceError>;
}

pub struct TerminalLogging {}

impl TerminalLogging {
    pub fn new() -> Self {
        Self {}
    }
}

impl LoggingService for TerminalLogging {
    fn write_log(&self, message: String) -> Result<(), ServiceError> {
        println!("{message}");

        Ok(())
    }
}
