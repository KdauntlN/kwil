# Kwil

## Description
Kwil is a simple and young library aimed for general logging purposes, written in Rust.

## Installation
Kwil is a library, and as such the easiest way to use it is through crates.io

## Usage
Kwil is simple to use and the main focus is on the setup. Creating a logger is as simple as:
```rust
use kwil;

fn main() {
    let mut logger = kwil::logger().build();
}
```

To create logs with this logger, you must add a handler, which includes a destination to write to and a formatter, detailing how those logs should be written.
```rust
use kwil;

fn main() {
    let mut logger = kwil::logger()
        .add_handler(
            std::io::stdout(),
            kwil::text()
        )
        .build();
}
```

The process to write a log is incredibly simple, only requiring one function call describing the severity of the log
```rust
logger.trace("This is a trace message");

logger.debug("This is a debug message");

logger.info("This is an info message");

logger.warning("This is a warning message");

logger.error("This is an error message");

logger.fatal("This is a fatal error message");
```