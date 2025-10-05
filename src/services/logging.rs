pub trait LoggingService {}

pub struct TerminalLogging {}

impl TerminalLogging {
    pub fn new() -> Self {
        Self {}
    }
}

impl LoggingService for TerminalLogging {}
