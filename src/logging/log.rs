use crate::formatting::formatter::Formatter;

#[derive(Debug, Clone)]
pub enum Level {
    Fatal,
    Error,
    Warning,
    Info,
    Debug,
    Trace
}

#[derive(Debug, Clone)]
pub struct Log<'a> {
    pub level: Level,
    pub msg: &'a str
}

impl<'a> Log<'a> {
    pub fn new(level: Level, msg: &'a str) -> Self {
        Self {
            level: level,
            msg: msg
        }
    }

    pub fn format<F: Formatter + ?Sized>(self, formatter: &Box<F>) -> String {
        formatter.format(self)
    }
}