use crate::logging::logger::Logger;

use crate::formatting::formatter::Formatter;

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

pub fn logger<F>(func: F) -> Logger
where
    F: FnOnce(&mut LoggerBuilder) -> ()
{
    let mut logger_builder = LoggerBuilder::new();

    func(&mut logger_builder);

    logger_builder.build()
}