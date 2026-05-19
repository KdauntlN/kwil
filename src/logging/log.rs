pub enum Level {
    Fatal,
    Error,
    Warning,
    Info,
    Debug,
    Trace
}

pub struct Log<'a> {
    level: Level,
    msg: &'a str
}

impl<'a> Log<'a> {
    pub fn new(level: Level, msg: &'a str) -> Self {
        Self {
            level: level,
            msg: msg
        }
    }
}