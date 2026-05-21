mod logging;
mod formatting;

pub use logging::builder::logger;
pub use formatting::text_formatter::plain_text;
pub use formatting::colour_text::coloured_text;