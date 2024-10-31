use crate::prelude::*;

#[derive(Default)]
pub struct MergeOptions {
    pub remove_deleted: bool,
}

/// Merge the given plugin into the master plugin.
///
#[allow(clippy::ptr_arg)]
pub fn merge_plugins(plugin_path: &PathBuf, master_path: &PathBuf, options: MergeOptions) -> Result<PluginData> {
    let mut plugin = PluginData::from_path(plugin_path)?;
    let master_name = plugin.header.ensure_master_present(master_path)?;

    let mut master = merge_masters(&plugin, master_path, master_name)?;

    remap_masters(&mut plugin, &master, master_name);
    remap_textures(&mut plugin, &master);
    plugin.merge_into(&mut master);

    master.remove_ignored();

    if options.remove_deleted {
        master.remove_deleted();
    }

    Ok(master)
}

/// Create a merged master from the given plugin's masters list.
///
/// Only `master_name` will be loaded in its entirety, others will only load the dialogue list.
///
fn merge_masters(plugin: &PluginData, master_path: &Path, master_name: &str) -> Result<PluginData> {
    let _guard = set_log_level(Level::WARN);

    let mut merged = default();
    let mut header = default();

    for (name, _) in &plugin.header.masters {
        let path = master_path.with_file_name(name);

        let mut master;

        if name.eq_ignore_ascii_case(master_name) {
            master = PluginData::from_path(&path)?;
            header = take(&mut master.header);
        } else {
            master = PluginData::from_path_dialogue_only(&path)?;
            master.set_all_ignored(true);
        }

        master.merge_into(&mut merged);
    }

    merged.header = header;

    Ok(merged)
}
