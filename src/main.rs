use merge_to_master::prelude::*;

use clap::{Arg, ArgAction, command};

// #[global_allocator]
// static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> Result<()> {
    let matches = command!()
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
                .long("list")
                .short('l')
                .help("A text file containing a list of plugins to merge (one per line).")
                .value_parser(into_file_path)
                .conflicts_with_all(["PLUGIN", "MASTER"])
                .requires("OUTPUT"),
            Arg::new("OUTPUT")
                .long("output")
                .help("The output path for the merged list. Required if --list is used.")
                .value_parser(into_path)
                .requires("LIST"),
            Arg::new("REMOVE-DELETED")
                .help("Remove all objects that are marked as DELETED.")
                .long("remove-deleted")
                .short('r')
                .action(ArgAction::SetTrue),
            Arg::new("OVERWRITE")
                .help("Overwrite <MASTER> without creating a backup.")
                .long("overwrite")
                .short('o')
                .action(ArgAction::SetTrue),
            Arg::new("PRESERVE-DUPLICATE-REFERENCES")
                .help("Preserve duplicate references, if not specified duplicates will be removed.")
                .long("preserve-duplicate-references")
                .action(ArgAction::SetTrue),
            Arg::new("APPLY-MOVED-REFERENCES")
                .help("Put 'moved references' into their the new cell's reference list. (Experimental)")
                .long("apply-moved-references")
                .action(ArgAction::SetTrue),
        ])
        .get_matches();

    // files
    let plugin_path = matches.get_one::<PathBuf>("PLUGIN");
    let master_path = matches.get_one::<PathBuf>("MASTER");
    let list_path = matches.get_one::<PathBuf>("LIST");
    let output_path = matches.get_one::<PathBuf>("OUTPUT");

    // flags
    let overwrite = matches.get_flag("OVERWRITE");
    let remove_deleted = matches.get_flag("REMOVE-DELETED");
    let apply_moved_references = matches.get_flag("APPLY-MOVED-REFERENCES");
    let preserve_duplicate_references = matches.get_flag("PRESERVE-DUPLICATE-REFERENCES");

    let (log_path, _guard) = init_logger()?;

    // Load plugin list if provided

    let list_path: Option<Vec<PathBuf>> = list_path.map(|list_path| {
        std::fs::read_to_string(list_path)
            .expect("Failed to read plugin list file")
            .lines()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(PathBuf::from_slash)
            .collect()
    });

    info!("Merging plugins...");

    let merge_options = MergeOptions {
        remove_deleted,
        apply_moved_references,
        preserve_duplicate_references,
    };

    let (output, merged) = match (list_path, output_path, plugin_path, master_path) {
        (Some(list_path), Some(output_path), ..) => {
            let merged = merge_load_order(&list_path)?;
            (output_path, merged)
        }
        (.., Some(plugin_path), Some(master_path)) => {
            let merged = merge_plugins(plugin_path, master_path, merge_options)?;
            (master_path, merged)
        }
        _ => {
            bail!(
                "Invalid arguments: either both PLUGIN and MASTER must be provided, or both LIST and OUTPUT must be provided."
            );
        }
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
