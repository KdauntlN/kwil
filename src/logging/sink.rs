use std::io::Write;

use crate::logging::log::Log;
use crate::formatting::formatter::Formatter;

pub struct Sink {
    output: Box<dyn Write + 'static>,
    formatter: Box<dyn Formatter>
}

impl Sink {
    pub fn new<T: Write + 'static, F: Formatter + 'static>(output: T, formatter: F) -> Self {
        Self {
            output: Box::new(output),
            formatter: Box::new(formatter)
        }
    }

    pub fn handle(&mut self, log: Log) {

        match self.output.write(log.format(&self.formatter).as_bytes()) {
            _ => return,
        }
    }
}