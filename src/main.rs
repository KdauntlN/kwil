use kwil;

fn main() {
    let mut logger = kwil::logger()
        .add_handler(
            std::io::stdout(),
            kwil::text(),
        )
        .add_handler(
            std::io::stderr(),
            kwil::text()
        )
        .build();

    logger.info("Nothing happened");
}