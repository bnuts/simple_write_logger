#[macro_use]
extern crate log;
extern crate simple_write_logger as logger;

mod test;

#[test]
fn writers() {
    use std::io::{stderr, stdout};

    let writers = vec![
        logger::Writer(Box::new(stdout())),
        logger::Writer(Box::new(stderr())),
    ];
    let expect = "DEBUG:tests\\writers.rs:16:DEBUG\n";
    test::test(writers, log::Level::Trace, expect, || debug!("DEBUG"));
}
