use crate::prelude::*;

use tes3::esp::{Cell, EditorId, TES3Object, TypeInfo};

/// Merge `self` into `target`.
///
pub trait MergeInto {
    fn merge_into(self, target: &mut Self);
}

impl MergeInto for PluginData {
    fn merge_into(self, target: &mut Self) {
        // Merge header
        target.header = self.header;

        // Merge objects
        for (key, object) in self.objects {
            let tag = object.tag_str();
            let id = object.editor_id();

            let entry = target.objects.entry(key);
            if let Entry::Occupied(entry) = entry {
                info!("Merging object to master: {tag} {id}");
                object.merge_into(entry.into_mut());
            } else {
                info!("Copying object to master: {tag} {id}");
                entry.insert(object);
            }
        }

        // Merge dialogue
        for (key, object) in self.dialogues {
            let entry = target.dialogues.entry(key);
            if let Entry::Occupied(entry) = entry {
                info!("Merging object to master: {}", entry.key());
                object.merge_into(entry.into_mut());
            } else {
                info!("Copying object to master: {}", entry.key());
                entry.insert(object);
            }
        }
    }
}

impl MergeInto for TES3Object {
    fn merge_into(self, target: &mut Self) {
        if let TES3Object::Cell(cell) = self {
            cell.merge_into(target.try_into().unwrap());
        } else {
            *target = self;
        }
    }
}

impl MergeInto for Cell {
    fn merge_into(self, target: &mut Self) {
        target.flags = self.flags;
        target.name = self.name;
        target.data = self.data;

        if self.region.is_some() {
            target.region = self.region;
        }
        if self.map_color.is_some() {
            target.map_color = self.map_color;
        }
        if self.water_height.is_some() {
            target.water_height = self.water_height;
        }
        if self.atmosphere_data.is_some() {
            target.atmosphere_data = self.atmosphere_data;
        }

        target.references.extend(self.references);
    }
}

impl MergeInto for DialogueGroup {
    fn merge_into(self, target: &mut Self) {
        target.dialogue = self.dialogue;
        for info in self.infos {
            target.insert_info(info);
        }
        target.repair_links();
    }
}
