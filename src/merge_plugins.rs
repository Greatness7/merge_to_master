use crate::prelude::*;

#[derive(Default)]
pub struct MergeOptions {
    pub remove_deleted: bool,
}

#[allow(clippy::ptr_arg)]
pub fn merge_plugins(plugin_path: &PathBuf, master_path: &PathBuf, options: MergeOptions) -> Result<PluginData> {
    let mut plugin = PluginData::from_path(plugin_path)?;
    let mut master = PluginData::from_path(master_path)?;

    let master_name = master_path
        .file_name()
        .and_then(OsStr::to_str)
        .expect("Invalid master path.");

    remap_masters(&mut plugin, &master, master_name);
    remap_textures(&mut plugin, &master);

    plugin.merge_into(&mut master);

    if options.remove_deleted {
        master.remove_deleted();
    }

    Ok(master)
}
