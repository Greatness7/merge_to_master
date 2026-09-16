use tes3::esp::Header;

use crate::prelude::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct MergeOptions {
    pub remove_deleted: bool,
    pub apply_moved_references: bool,
    pub preserve_duplicate_references: bool,
}

impl MergeOptions {
    fn apply(self, merged: &mut PluginData) {
        if self.remove_deleted {
            merged.remove_deleted();
        }

        if self.apply_moved_references {
            merged.cells.apply_moved_references();
        }

        if !self.preserve_duplicate_references {
            merged.cells.remove_duplicate_references();
        }
    }
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

    options.apply(&mut master);

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

/// Create a merged plugin from a list of plugin paths (in load order).
///
/// Use when you need the fully-resolved game state for a given load order.
///
pub fn merge_load_order(plugin_paths: &[PathBuf], options: MergeOptions) -> Result<PluginData> {
    let _guard = set_log_level(Level::WARN);

    let masters = Header::collect_masters(plugin_paths)?;
    let masters_remap = Header::build_master_remap(&masters);

    let mut merged = PluginData::default();

    for (i, path) in plugin_paths.iter().enumerate() {
        let mut plugin = PluginData::from_path_remap_masters(path, i, &masters_remap)?;
        plugin.remap_textures(&merged);
        plugin.merge_into(&mut merged);
    }

    merged.header.file_type = tes3::esp::FileType::Esm;
    merged.header.masters = masters;

    options.apply(&mut merged);

    merged.remove_ignored();

    Ok(merged)
}

/// Parallel version of [`merge_load_order`].
///
pub fn par_merge_load_order(plugin_paths: &[PathBuf], options: MergeOptions) -> Result<PluginData> {
    let _guard = set_log_level(Level::WARN);

    let masters = Header::collect_masters(plugin_paths)?;
    let masters_remap = Header::build_master_remap(&masters);

    let mut merged = PluginData::default();

    // Plugins are loaded and preprocessed in parallel batches, then merged
    // sequentially in the correct order. This bounds memory usage to roughly
    // one batch worth of plugins beyond the merged accumulator.

    let batch_size = rayon::current_num_threads().max(8);
    let mut offset = 0;

    for batch in plugin_paths.chunks(batch_size) {
        let loaded: Vec<PluginData> = batch
            .par_iter()
            .enumerate()
            .map(|(i, path)| PluginData::from_path_remap_masters(path, offset + i, &masters_remap))
            .collect::<Result<_>>()?;

        for mut plugin in loaded {
            plugin.remap_textures(&merged);
            plugin.merge_into(&mut merged);
        }

        offset += batch.len();
    }

    merged.header.file_type = tes3::esp::FileType::Esm;
    merged.header.masters = masters;

    options.apply(&mut merged);

    merged.remove_ignored();

    Ok(merged)
}
