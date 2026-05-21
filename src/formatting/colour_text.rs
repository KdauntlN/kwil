use crate::{
    formatting::formatter::Formatter,
    logging::log::{
        Log,
        Level,
    }
};

pub struct ColourTextFormatter;

impl ColourTextFormatter {
    fn new() -> Self {
        Self {

        }
    }
}

impl Formatter for ColourTextFormatter {
    fn format(&self, log: Log) -> String {
        let mut message = String::new();

        use Level as L;
        message.push_str(match log.level {
            L::Trace => "\x1b[36m[TRACE]\x1b[0m   ",
            L::Debug => "\x1b[35m[DEBUG]\x1b[0m   ",
            L::Info => "\x1b[32m[INFO]\x1b[0m    ",
            L::Warning => "\x1b[33m[WARNING]\x1b[0m ",
            L::Error => "\x1b[31m[ERROR]\x1b[0m   ",
            L::Fatal => "\x1b[91m[FATAL]\x1b[0m   "
        });

        message.push_str(log.msg);
        message.push('\n');

        message
    }
}

pub fn coloured_text() -> ColourTextFormatter {
    ColourTextFormatter::new()
}