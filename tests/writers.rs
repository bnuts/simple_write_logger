#[macro_use] extern crate log;
extern crate simple_write_logger as logger;

use std::env::temp_dir;
use std::fs::{File, remove_file};
use std::io::{Read, stdout, stderr};

#[test]
fn writers() {
    let mut t = temp_dir();
    t.push("simple_write_logger.txt");
    let temp = t.to_str().unwrap();

    {
        let file = File::create(temp);
        assert!(file.is_ok());

        let writers = vec![
            logger::Writer(Box::new(file.unwrap())),
            logger::Writer(Box::new(stdout())),
            logger::Writer(Box::new(stderr())),
        ];
        assert!(logger::init(writers).is_ok());
    }

    debug!("DEBUG");

    {
        let file = File::open(temp);
        assert!(file.is_ok());

        let expect = "writers:tests\\writers.rs:26:DEBUG:\tDEBUG\n";
        let mut file_str = String::new();
        let _ = file.unwrap().read_to_string(&mut file_str);
        assert_eq!(file_str, expect);
    }

    assert!(remove_file(temp).is_ok());
}
