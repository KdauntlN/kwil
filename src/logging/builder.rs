use crate::logging::sink::Sink;

use std::io::Write;

pub struct LoggerBuilder {
    handlers: Vec<Sink>
}

impl LoggerBuilder {
    fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler<T: Write + 'static>(mut self, file: T) -> Self {
        self.handlers.push(Sink::new(file));

        Self {
            handlers: self.handlers,
        }
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
        assert!(logger.handlers.len() > 0);
    }
}