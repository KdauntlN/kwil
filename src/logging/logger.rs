use crate::logging::builder::LoggerBuilder;
use crate::logging::log::{Log, Level};

pub struct Logger {
    sinks: Vec<Sink>,
}

impl Logger {
    pub fn new(builder: LoggerBuilder) -> Self {
        Self {
            sinks: builder.sinks
        }
    }

    pub fn trace(&mut self, msg: &str) {
        let log: Log<'_> = Log::new(Level::Trace, msg);

        for sink in self.sinks.iter_mut() {
            sink.handle(log.clone());
        }
    }

    pub fn debug(&mut self, msg: &str) {
        let log: Log<'_> = Log::new(Level::Debug, msg);

        for sink in self.sinks.iter_mut() {
            sink.handle(log.clone());
        }
    }

    pub fn info(&mut self, msg: &str) {
        let log: Log<'_> = Log::new(Level::Info, msg);

        for sink in self.sinks.iter_mut() {
            sink.handle(log.clone());
        };
    }

    pub fn warning(&mut self, msg: &str) {
        let log: Log<'_> = Log::new(Level::Warning, msg);

        for sink in self.sinks.iter_mut() {
            sink.handle(log.clone());
        };
    }

    pub fn error(&mut self, msg: &str) {
        let log: Log<'_> = Log::new(Level::Error, msg);

        for sink in self.sinks.iter_mut() {
            sink.handle(log.clone());
        };
    }

    pub fn fatal(&mut self, msg: &str) {
        let log: Log<'_> = Log::new(Level::Fatal, msg);

        for sink in self.sinks.iter_mut() {
            sink.handle(log.clone());
        };
    }
}