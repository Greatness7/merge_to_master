use crate::prelude::*;

pub fn backup(path: &Path) -> Option<PathBuf> {
    let parent = path.parent()?.as_os_str();
    let extension = path.extension()?.to_str()?;
    let file_stem = path.file_stem()?.to_str()?;

    let executable = std::env::current_exe().ok()?;
    let executable_stem = executable.file_stem()?;

    let backups_dir = std::borrow::Cow::from_backslash("backups\\");
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
