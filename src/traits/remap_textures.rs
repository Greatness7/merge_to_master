use std::sync::atomic::{AtomicU32, Ordering};

use tes3::esp::{LandscapeTexture, TES3Object};

use crate::prelude::*;

pub trait RemapTextures {
    /// Remap the texture indices in the plugin to be compatible with those in the
    /// master plugin.
    ///
    /// This is necessary as texture indices inside plugins are "local" to the file
    /// and will differ between plugins even if they actualy mean the same texture.
    ///
    fn remap_textures(&mut self, master: &PluginData);
}

impl RemapTextures for PluginData {
    fn remap_textures(&mut self, master: &PluginData) {
        if let Some(index_remap) = get_index_remap(self, master) {
            apply_index_remap(self, &index_remap);
        }
    }
}

type IndexRemap = HashMap<u16, u16>;

fn get_index_remap(this: &mut PluginData, master: &PluginData) -> Option<IndexRemap> {
    let next_index = AtomicU32::new(next_texture_index(master)?);

    let index_remap = this
        .objects
        .par_iter_mut()
        .filter_map(|(key, object)| {
            use TES3Object::LandscapeTexture;

            let LandscapeTexture(texture) = object else {
                return None;
            };

            let old_index = texture.index;
            let new_index = match master.objects.get(key) {
                Some(LandscapeTexture(master)) => master.index,
                _ => next_index.fetch_add(1, Ordering::SeqCst),
            };

            // Remap is unncessary if indices are already identical.
            if old_index == new_index {
                return None;
            }

            info!("Remapping texture index: ({old_index} -> {new_index}) {}", texture.id);
            texture.index = new_index;

            // We need to +1 for remap lookups because 0 is reserved for "no texture".
            // Also ensure we can fit into a u16 as that's what the landscapes expect.
            assert!(old_index < 0xFFFF && new_index < 0xFFFF);

            #[allow(clippy::cast_possible_truncation)] // Ensured by the prior assert.
            Some((old_index as u16 + 1, new_index as u16 + 1))
        })
        .collect();

    Some(index_remap)
}

fn next_texture_index(this: &PluginData) -> Option<u32> {
    this.objects
        .values()
        .filter_map(|object| {
            let texture: &LandscapeTexture = object.try_into().ok()?;
            Some(texture.index)
        })
        .max()
        .map(|i| i + 1)
}

fn apply_index_remap(this: &mut PluginData, index_remap: &IndexRemap) {
    this.cells.exteriors.par_values_mut().for_each(|exterior| {
        if let Some(landscape) = exterior.landscape.as_mut() {
            for old_index in landscape.texture_indices.data.as_flattened_mut() {
                if let Some(new_index) = index_remap.get(old_index) {
                    *old_index = *new_index;
                }
            }
        }
    });
}
