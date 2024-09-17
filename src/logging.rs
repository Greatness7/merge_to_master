use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use anyhow::{Context, Result};

use path_slash::PathBufExt;

use tracing::subscriber::DefaultGuard;
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};
use tracing_subscriber::prelude::*;

pub use tracing::{debug, error, info, trace, warn, Level};

/// Set the global log level for the current scope.
///
pub fn set_log_level(level: Level) -> DefaultGuard {
    tracing_subscriber::fmt().with_max_level(level).set_default()
}

/// Initialize the logger and return the log file path and guard.
///
pub fn init_logger() -> Result<(PathBuf, WorkerGuard)> {
    let path = PathBuf::from_backslash(".\\merge_to_master.log");

    let file = File::create(&path).with_context(|| format!("{path:?}"))?;

    let (writer, guard) = NonBlocking::new(BufWriter::new(file));

    tracing_subscriber::fmt()
        .with_writer(writer)
        .with_max_level(Level::INFO)
        .with_ansi(false)
        .with_file(false)
        .with_level(false)
        .with_target(false)
        .without_time()
        .init();

    Ok((path, guard))
}
