extern crate log;

use std::io::{Write, stderr};
use std::sync::Mutex;

use log::{Log, LogLevel, LogMetadata, LogRecord, SetLoggerError};

pub struct Writer(pub Box<Write + Send>);

struct Logger {
    writers: Mutex<Vec<Writer>>,
    level: LogLevel,
}

impl Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool { metadata.level() <= self.level }

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
            let w = &mut writers[i].0;
            let _ = writeln!(w, "{}:{}:{}:{}", rec.level(), loc.file(), loc.line(), rec.args());
        }
    }
}

pub fn init(writers: Vec<Writer>, level: LogLevel) -> Result<(), SetLoggerError> {
    log::set_logger(|max_level| {
        max_level.set(level.to_log_level_filter());
        let logger = Logger {
            writers: Mutex::new(writers),
            level: level,
        };
        Box::new(logger)
    })
}
