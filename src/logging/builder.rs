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

    pub fn add_handler<T: Write + 'static>(mut self, file: T) -> Self {
        self.sinks.push(Sink::new(file));

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_handler_test() {
        let logger = logger().add_handler(std::io::stdout());
        assert!(logger.sinks.len() > 0);
    }
}