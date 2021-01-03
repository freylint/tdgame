use std::fs::File;

pub use log::{debug, error, info, trace};
use simplelog::*;

pub fn init_logger() {
    // Setup logger
    #[cfg(logging)]
        CombinedLogger::init(vec![
        // Terminal Output
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed),
        // File Output
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("debug.log").unwrap(),
        ),
    ])
        .unwrap();
}
