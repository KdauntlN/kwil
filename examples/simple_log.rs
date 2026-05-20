use kwil;

fn main() {
    // Create a Logger and assign a sink to stdout
    let mut logger = kwil::logger()
        .add_handler(
            std::io::stdout(), 
            kwil::text()
        )
        .build();

    // Log messages of various severities
    logger.trace("This is a trace message");

    logger.debug("This is a debug message");

    logger.info("This is an info message");

    logger.warning("This is a warning message");

    logger.error("This is an error message");

    logger.fatal("This is a fatal error message");
}