use std::sync::atomic::{AtomicU32, Ordering};

use tes3::esp::TES3Object;

use crate::prelude::*;

type IndexRemap = HashMap<u16, u16>;

pub fn remap_textures(plugin: &mut PluginData, master: &PluginData) {
    if let Some(index_remap) = get_index_remap(plugin, master) {
        apply_index_remap(plugin, &index_remap);
    }
}

fn get_index_remap(this: &mut PluginData, master: &PluginData) -> Option<IndexRemap> {
    let next_index = AtomicU32::new(master.next_texture_index()?);

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

fn apply_index_remap(this: &mut PluginData, index_remap: &IndexRemap) {
    this.objects.par_values_mut().for_each(|object| {
        if let TES3Object::Landscape(landscape) = object {
            for old_index in landscape.texture_indices.data.as_flattened_mut() {
                if let Some(new_index) = index_remap.get(old_index) {
                    *old_index = *new_index;
                }
            }
        }
    });
}
