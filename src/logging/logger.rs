use crate::logging::{
    sink::Sink,
    builder::LoggerBuilder,
    log::{
        Log,
        Level,
    },
};

#[derive(Debug)]
pub struct Logger {
    sinks: Vec<Sink>,
}

impl Logger {
    pub fn new(builder: LoggerBuilder) -> Self {
        Self {
            sinks: builder.sinks
        }
    }

    pub fn info(&mut self, msg: &str) {
        let log: Log<'_> = Log::new(Level::Info, msg);

        for sink in self.sinks.iter_mut() {
            sink.handle(log.clone());
        };
    }
}