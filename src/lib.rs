extern crate log;

use std::io::{Write, stderr};
use std::sync::Mutex;

use log::{Log, LogLevelFilter, LogMetadata, LogRecord, SetLoggerError};

pub struct Writer(pub Box<Write + Send>);

struct Logger {
    writers: Mutex<Vec<Writer>>,
}

impl Log for Logger {
    fn enabled(&self, _: &LogMetadata) -> bool {
        true
    }

    fn log(&self, rec: &LogRecord) {
        let mut writers = match self.writers.lock() {
            Ok(writers) => writers,
            Err(err) => {
                let _ = writeln!(stderr(), "{:?}", err);
                return;
            },
        };
        let loc = rec.location();
        for i in 0..writers.len() {
            let _ = writeln!(
                writers[i].0,
                "{}:{}:{}:{}",
                rec.level(), loc.file(), loc.line(), rec.args());
        }
    }
}

pub fn init(writers: Vec<Writer>) -> Result<(), SetLoggerError> {
    log::set_logger(|max_level| {
        max_level.set(LogLevelFilter::max());
        Box::new(Logger {writers: Mutex::new(writers)})
    })
}
