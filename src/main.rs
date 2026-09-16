use merge_to_master::prelude::*;

use clap::{Arg, ArgAction, ArgMatches, command};

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> Result<()> {
    let mut matches = command!()
        .arg_required_else_help(true)
        .args(&[
            Arg::new("PLUGIN")
                .help("The plugin that will be merged into <MASTER>.")
                .value_parser(into_file_path)
                .required_unless_present("LIST"),
            Arg::new("MASTER")
                .help("The master that <PLUGIN> will be merged into.")
                .value_parser(into_file_path)
                .required_unless_present("LIST"),
            Arg::new("LIST")
                .help("A text file listing plugins to merge, in load order (one per line).")
                .long("list")
                .value_parser(into_file_path)
                .conflicts_with_all(["PLUGIN", "MASTER"])
                .requires("OUTPUT"),
            Arg::new("OUTPUT")
                .help("The path to save the merged <LIST> to.")
                .long("output")
                .value_parser(into_path)
                .requires("LIST"),
            Arg::new("REMOVE-DELETED")
                .help("Remove all objects that are marked as DELETED.")
                .long("remove-deleted")
                .short('r')
                .action(ArgAction::SetTrue),
            Arg::new("OVERWRITE")
                .help("Overwrite the output file without creating a backup.")
                .long("overwrite")
                .short('o')
                .action(ArgAction::SetTrue),
            Arg::new("PRESERVE-DUPLICATE-REFERENCES")
                .help("Preserve duplicate references, if not specified duplicates will be removed.")
                .long("preserve-duplicate-references")
                .action(ArgAction::SetTrue),
            Arg::new("APPLY-MOVED-REFERENCES")
                .help("Put 'moved references' into the new cell's reference list. (Experimental)")
                .long("apply-moved-references")
                .action(ArgAction::SetTrue),
        ])
        .get_matches();

    let mode = Mode::from_matches(&mut matches);
    let overwrite = matches.get_flag("OVERWRITE");
    let options = MergeOptions {
        remove_deleted: matches.get_flag("REMOVE-DELETED"),
        apply_moved_references: matches.get_flag("APPLY-MOVED-REFERENCES"),
        preserve_duplicate_references: matches.get_flag("PRESERVE-DUPLICATE-REFERENCES"),
    };

    let (log_path, _guard) = init_logger()?;

    info!("Merging plugins...");

    let (merged, output) = match &mode {
        Mode::Plugin { plugin, master } => (merge_plugins(plugin, master, options)?, master),
        Mode::LoadOrder { list, output } => (merge_load_order(&read_list(list)?, options)?, output),
    };

    if !overwrite && output.exists() {
        info!("Creating backup...");
        if backup(output).is_none() {
            bail!("Failed to create backup.");
        }
    }

    info!("Saving results...");

    merged.save_path(output)?;

    info!("Finished!");

    eprintln!("Merge Successful: {}", output.display());
    eprintln!("Log available at: {}", log_path.display());

    Ok(())
}

enum Mode {
    /// Merge `plugin` into `master`, saving over `master`.
    Plugin { plugin: PathBuf, master: PathBuf },
    /// Merge every plugin listed in `list` (in load order), saving to `output`.
    LoadOrder { list: PathBuf, output: PathBuf },
}

impl Mode {
    fn from_matches(matches: &mut ArgMatches) -> Self {
        match (
            matches.remove_one("PLUGIN"),
            matches.remove_one("MASTER"),
            matches.remove_one("LIST"),
            matches.remove_one("OUTPUT"),
        ) {
            (Some(plugin), Some(master), None, None) => Mode::Plugin { plugin, master },
            (None, None, Some(list), Some(output)) => Mode::LoadOrder { list, output },
            _ => unreachable!(),
        }
    }
}

fn read_list(path: &Path) -> Result<Vec<PathBuf>> {
    let Ok(text) = std::fs::read_to_string(path) else {
        bail!("Failed to read list: {}", path.display())
    };
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(into_file_path)
        .collect()
}

fn into_path(arg: &str) -> Result<PathBuf> {
    Ok(PathBuf::from_slash(arg))
}

fn into_file_path(arg: &str) -> Result<PathBuf> {
    let path = PathBuf::from_slash(arg);
    if !path.is_file() {
        bail!("Invalid file path: {}", path.display());
    }
    Ok(path)
}
