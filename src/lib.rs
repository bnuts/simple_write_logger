extern crate log;

use std::{
    io::{stderr, Write},
    sync::Mutex,
};

use log::{Level, Log, Metadata, Record, SetLoggerError};

pub struct Writer(pub Box<dyn Write + Send>);

struct Logger {
    writers: Mutex<Vec<Writer>>,
    level: Level,
    write_location: bool,
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, rec: &Record) {
        let mut writers = match self.writers.lock() {
            Ok(writers) => writers,
            Err(err) => {
                let _ = writeln!(stderr(), "{:?}", err);
                return;
            },
        };
        for i in 0..writers.len() {
            let w = &mut writers[i].0;
            if self.write_location {
                let _ = writeln!(
                    w,
                    "{}:{}:{}:{}",
                    rec.level(),
                    rec.file().unwrap(),
                    rec.line().unwrap(),
                    rec.args()
                );
            } else {
                let _ = writeln!(w, "{}", rec.args());
            }
        }
    }

    fn flush(&self) {}
}

pub fn init(
    writers: Vec<Writer>,
    level: Level,
    write_location: bool,
) -> Result<(), SetLoggerError> {
    log::set_max_level(level.to_level_filter());
    let logger = Logger {
        writers: Mutex::new(writers),
        level: level,
        write_location,
    };
    log::set_boxed_logger(Box::new(logger))
}
