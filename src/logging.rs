use std::path::PathBuf;

use path_slash::PathBufExt;

pub use flexi_logger::LoggerHandle;
pub use log::{debug, error, info, trace, warn};

pub fn start_logger() -> anyhow::Result<(PathBuf, LoggerHandle)> {
    use flexi_logger::*;

    let target = PathBuf::from_backslash(".\\merge_to_master.log");
    let handle = Logger::with(LogSpecification::info())
        .format(|w, _, r| write!(w, "{}", r.args()))
        .log_to_file(FileSpec::try_from(&target)?)
        .write_mode(WriteMode::Async)
        .start()?;

    Ok((target, handle))
}
