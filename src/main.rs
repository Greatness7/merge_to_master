use merge_to_master::prelude::*;

use clap::{command, Arg, ArgAction};

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
        ])
        .get_matches();

    // files
    let plugin_path = matches.get_one("PLUGIN").unwrap();
    let master_path = matches.get_one("MASTER").unwrap();

    // flags
    let overwrite = matches.get_flag("OVERWRITE");
    let remove_deleted = matches.get_flag("REMOVE-DELETED");

    let (log_path, _guard) = init_logger()?;

    info!("Merging plugins...");

    let merged = merge_plugins(
        plugin_path,
        master_path,
        MergeOptions { remove_deleted }, //
    )?;

    // backup
    info!("Creating backup...");

    if !overwrite && backup(master_path).is_none() {
        bail!("Failed to create backup.");
    };

    // save
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

pub fn backup(path: &Path) -> Option<PathBuf> {
    let parent = path.parent()?.as_os_str();
    let extension = path.extension()?.to_str()?;
    let file_stem = path.file_stem()?.to_str()?;

    let executable = std::env::current_exe().ok()?;
    let executable_stem = executable.file_stem()?;

    let backups_dir = Cow::from_backslash("backups\\");
    let capacity = parent.len() + backups_dir.as_os_str().len() + executable_stem.len() + 7;

    let mut backup_path = PathBuf::with_capacity(capacity);
    backup_path.push(parent);
    backup_path.push(backups_dir);
    backup_path.push(executable_stem);

    info!("Creating backup directory: {}", backup_path.display());
    std::fs::create_dir_all(&backup_path).ok()?;

    info!("Finding next available backup file name...");
    let i = std::fs::read_dir(&backup_path)
        .ok()?
        .filter_map(|entry| {
            // parse version number from a file
            // example: "Master.108.esm" -> 108
            entry
                .ok()?
                .file_name()
                .to_str()?
                .strip_prefix(file_stem)?
                .strip_suffix(extension)?
                .trim_matches('.')
                .parse::<usize>()
                .ok()
        })
        .max()
        .map_or(0, |i| i + 1);

    backup_path.push(format!("{file_stem}.{i:03}.{extension}"));
    if backup_path.exists() {
        info!("Cannot create backup: {}", backup_path.display());
        return None;
    }

    info!("Creating backup file: {}", backup_path.display());
    std::fs::rename(path, &backup_path).ok()?;
    Some(backup_path)
}
