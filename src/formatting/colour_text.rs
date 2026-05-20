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
            L::Trace => "[TRACE]: ",
            L::Debug => "[DEBUG]: ",
            L::Info => "[INFO]: ",
            L::Warning => "[WARNING]: ",
            L::Error => "[ERROR]: ",
            L::Fatal => "\x1b[31m[FATAL]:\x1b[0m "
        });

        message.push_str(log.msg);
        message.push('\n');

        message
    }
}

pub fn coloured_text() -> ColourTextFormatter {
    ColourTextFormatter::new()
}