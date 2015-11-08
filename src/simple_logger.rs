use log::{self, LogRecord, LogMetadata, LogLevelFilter, SetLoggerError};
struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, _metadata: &LogMetadata) -> bool {
        true // metadata.level() <= LogLevel::Trace
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }
}


pub fn init(verbose: bool) -> Result<(), SetLoggerError> {
    let level = if verbose {
        LogLevelFilter::Debug
    } else {
        LogLevelFilter::Info
    };

    log::set_logger(|max_log_level| {
        max_log_level.set(level);
        Box::new(SimpleLogger)
    })
}
