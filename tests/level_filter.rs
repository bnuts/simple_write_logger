#[macro_use]
extern crate log;
extern crate simple_write_logger as logger;

mod test;

#[test]
fn level_filter() {
    let writers = Vec::new();
    let expect = "";
    test::test(writers, log::Level::Info, expect, || debug!("DEBUG"));
}
