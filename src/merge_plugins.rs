use crate::prelude::*;

#[derive(Default)]
pub struct MergeOptions {
    pub remove_deleted: bool,
    pub apply_moved_references: bool,
    pub preserve_duplicate_references: bool,
}

/// Merge the given plugin into the master plugin.
///
#[allow(clippy::ptr_arg)]
pub fn merge_plugins(plugin_path: &PathBuf, master_path: &PathBuf, options: MergeOptions) -> Result<PluginData> {
    let mut plugin = PluginData::from_path(plugin_path)?;
    let master_name = plugin.header.ensure_master_present(master_path)?;

    let mut master = merge_masters(&plugin, master_path, master_name)?;

    plugin.remap_masters(&master, master_name);
    plugin.remap_textures(&master);
    plugin.merge_into(&mut master);

    if options.remove_deleted {
        master.remove_deleted();
    }

    if options.apply_moved_references {
        master.cells.apply_moved_references();
    }

    if !options.preserve_duplicate_references {
        master.cells.remove_duplicate_references();
    }

    master.remove_ignored();

    Ok(master)
}

/// Create a merged master from the given plugin's masters list.
///
/// Only `master_name` will be loaded in its entirety, others load only types needed for merge logic.
///
fn merge_masters(plugin: &PluginData, master_path: &Path, master_name: &str) -> Result<PluginData> {
    let _guard = set_log_level(Level::WARN);

    let mut merged = default();
    let mut header = default();

    let mut path = master_path.to_owned();

    for (name, _) in &plugin.header.masters {
        path.set_file_name(name);

        let mut master;

        if name.eq_ignore_ascii_case(master_name) {
            master = PluginData::from_path(&path)?;
            header = std::mem::take(&mut master.header);
        } else {
            master = PluginData::from_path_partial(&path)?;
            master.set_all_ignored(true);
        }

        master.merge_into(&mut merged);
    }

    merged.header = header;

    Ok(merged)
}
