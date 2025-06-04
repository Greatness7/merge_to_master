use merge_to_master::prelude::*;

use clap::{Arg, ArgAction, command};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> Result<()> {
    let matches = command!()
        .arg_required_else_help(true)
        .args(&[
            Arg::new("PLUGIN")
                .help("The plugin that will be merged into <MASTER>.")
                .value_parser(into_file_path)
                .required(true),
            Arg::new("MASTER")
                .help("The master that <PLUGIN> will be merged into.")
                .value_parser(into_file_path)
                .required(true),
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
    let plugin_path = matches.get_one("PLUGIN").unwrap();
    let master_path = matches.get_one("MASTER").unwrap();

    // flags
    let overwrite = matches.get_flag("OVERWRITE");
    let remove_deleted = matches.get_flag("REMOVE-DELETED");
    let apply_moved_references = matches.get_flag("APPLY-MOVED-REFERENCES");
    let preserve_duplicate_references = matches.get_flag("PRESERVE-DUPLICATE-REFERENCES");

    let (log_path, _guard) = init_logger()?;

    info!("Merging plugins...");

    let merged = merge_plugins(
        plugin_path,
        master_path,
        MergeOptions {
            remove_deleted,
            apply_moved_references,
            preserve_duplicate_references,
        },
    )?;

    if !overwrite {
        info!("Creating backup...");
        if backup(master_path).is_none() {
            bail!("Failed to create backup.");
        }
    }

    info!("Saving results...");

    merged.save_path(master_path)?;

    info!("Finished!");

    eprintln!("Merge Successful: {}", master_path.display());
    eprintln!("Log available at: {}", log_path.display());

    Ok(())
}

fn into_file_path(arg: &str) -> Result<PathBuf> {
    let path = PathBuf::from_slash(arg);
    if !path.is_file() {
        bail!("Invalid file path: {}", path.display());
    }
    Ok(path)
}
