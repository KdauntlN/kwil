use crate::logging::log::Log;

pub trait Formatter {
    fn format(&self, log: Log) -> String;
}