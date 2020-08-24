extern crate log;
extern crate simple_write_logger as logger;

pub fn test<F>(mut writers: Vec<logger::Writer>, level: log::Level, expect: &str, f: F)
where
    F: Fn(),
{
    use std::{
        env::temp_dir,
        fs::{remove_file, File},
        io::Read,
    };

    let mut t = temp_dir();
    t.push("simple_write_logger.txt");
    let temp = t.to_str().unwrap();

    {
        let file = File::create(temp).unwrap();
        writers.push(logger::Writer(Box::new(file)));
        assert!(logger::init(writers, level, true).is_ok());
    }

    f();

    {
        let mut file = File::open(temp).unwrap();
        let mut file_str = String::new();
        let _ = file.read_to_string(&mut file_str);
        assert_eq!(file_str, expect);
    }

    assert!(remove_file(temp).is_ok());
}
