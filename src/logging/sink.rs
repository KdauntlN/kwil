use std::io::Write;

pub struct Sink {
    output: Box<dyn Write + 'static>,
}

impl Sink {
    pub fn new<T: Write + 'static>(output: T) -> Self {
        Self {
            output: Box::new(output),
        }
    }
}