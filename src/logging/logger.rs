use std::error::Error;

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

    pub fn info(&self, msg: &str) -> Result<(), Box<dyn Error>> {
        let log = Log::new(Level::Info, msg);

        for sink in &self.sinks {
            
        }
    
        Ok(())
    }
}