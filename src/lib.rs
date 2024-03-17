//! # Microlog
//! Dead simple log subscriber, coming in at only `38` meaningful lines of code.
//!
//! - Just prints the `Level` (in color) and the message, nothing more nothing less.
//!
//! - All functions respect the `RUST_LOG` env variable to override the log level at runtime.
//!
//! - All functions support the `NO_COLOR` and `CLICOLOR` env variables to disable/change colored output at runtime.
//! See the [colored](https://crates.io/crates/colored) crate for details.
//!
//! # Features
//! - `no-color`: This feature allows you to completely compile out coloring code.
//!
//! # Example
//! ```
//! microlog::init(microlog::LevelFilter::Trace);
//!
//! log::trace!("Trace test");
//! log::debug!("Debug test");
//! log::info!("Info test");
//! log::warn!("Info test");
//! log::error!("Info test");
//! ```

use colored::Colorize;
use log::Level;
use std::env;

pub use log::LevelFilter;

struct MicroLog {}

/// Sets the global logger to microlog. This function should be called very early in the program
/// and must only be called once.
///
/// Respects the `RUST_LOG` env var.
///
/// # Panics
/// if the call to `log::set_logger` fails.
/// This most likely means that another logger has already been set.
#[inline]
pub fn init(level: LevelFilter) {
    try_init(level).expect("Failed to set logger. Did you already set a logger?");
}

/// Attempts to set the global logger to microlog. This function should be called very early in the program.
///
/// Respects the `RUST_LOG` env var.
///
/// # Errors
/// if the logger cannot be set
pub fn try_init(mut level: LevelFilter) -> Result<(), log::SetLoggerError> {
    static LOGGER: MicroLog = MicroLog {};

    if let Ok(rust_log_var) = env::var("RUST_LOG") {
        if let Ok(rust_log_var) = rust_log_var.parse::<LevelFilter>() {
            level = rust_log_var;
        }
    }

    log::set_logger(&LOGGER)?;
    log::set_max_level(level);

    Ok(())
}

impl log::Log for MicroLog {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let lvl = match record.level() {
                Level::Error => "Error".red(),
                Level::Warn => "Warn ".yellow(),
                Level::Info => "Info ".green(),
                Level::Debug => "Debug".blue(),
                Level::Trace => "Trace".white(),
            };
            eprintln!("{} {}", lvl, record.args());
        }
    }

    fn flush(&self) {}
}
