use time;
use log;
use std;

use log::{LogRecord, LogLevel, LogMetadata, Log, SetLoggerError, LogLevelFilter};
use std::io::Write;

/// Custom Logger
pub struct CustomLogger;

impl log::Log for CustomLogger {

    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Trace
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            match writeln!(&mut std::io::stderr(),
            "{}:{}:{} {}",
            time::strftime("%Y-%m-%d %H:%M:%S %Z", &time::now()).unwrap(),
            record.level(),
            record.location().module_path(),
            record.args()) {
                Err(e) => panic!("failed to log: {}", e),
                Ok(_) => {}
            }
        }
    }

}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(LogLevelFilter::Trace);
        Box::new(CustomLogger)
    })
}
