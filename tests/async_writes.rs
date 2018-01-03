#[macro_use]
extern crate log;
extern crate simple_write_logger as logger;

mod test;

const TEST_COUNT: usize = 5;

#[test]
fn async_writes() {
    use std::io::{stderr, stdout};
    use std::thread;

    let writers = vec![
        logger::Writer(Box::new(stdout())),
        logger::Writer(Box::new(stderr())),
    ];
    let expect = "DEBUG:tests\\async_writes.rs:22:DEBUG\n".repeat(TEST_COUNT);
    test::test(writers, log::Level::Debug, &expect, || {
        let mut threads = Vec::new();
        for _ in 0..TEST_COUNT {
            threads.push(thread::spawn(|| debug!("DEBUG")));
        }
        for t in threads {
            let _ = t.join();
        }
    });
}
