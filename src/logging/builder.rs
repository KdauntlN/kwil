use crate::logging::formatter::Formatter;
use crate::logging::sink::Sink;
use crate::logging::logger::Logger;

use std::io::Write;

pub struct LoggerBuilder {
    pub sinks: Vec<Sink>
}

impl LoggerBuilder {
    fn new() -> Self {
        Self {
            sinks: Vec::new(),
        }
    }

    pub fn add_handler<T, F>(mut self, file: T, formatter: F) -> Self
    where
        T: Write + 'static,
        F: Formatter + 'static
    {
        self.sinks.push(Sink::new(file, formatter));

        Self {
            sinks: self.sinks,
        }
    }

    pub fn build(self) -> Logger {
        Logger::new(self)
    }
}

pub fn logger() -> LoggerBuilder {
    LoggerBuilder::new()
}