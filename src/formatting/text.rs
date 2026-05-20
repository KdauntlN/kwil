use crate::{
    formatting::formatter::Formatter,
    logging::log::{
        Log,
        Level,
    }
};

pub struct TextFormatter;

impl TextFormatter {
    fn new() -> Self {
        Self {

        }
    }
}

impl Formatter for TextFormatter {
    fn format(&self, log: Log) -> String {
        let mut message = String::new();

        use Level as L;
        message.push_str(match log.level {
            L::Trace => "[TRACE]: ",
            L::Debug => "[DEBUG]: ",
            L::Info => "[INFO]: ",
            L::Warning => "[WARNING]: ",
            L::Error => "[ERROR]: ",
            L::Fatal => "[FATAL]: "
        });

        message.push_str(log.msg);
        message.push('\n');

        message
    }
}

pub fn plain_text() -> TextFormatter {
    TextFormatter::new()
}