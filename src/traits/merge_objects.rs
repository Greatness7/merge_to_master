use tes3::esp::{Cell, EditorId, TES3Object, TypeInfo};

use crate::prelude::*;

pub trait MergeInto {
    /// Merge `self` into `target`.
    ///
    /// Objects from `self` will typically overwrite objects in `target`.
    /// Certain types like Cells and Dialogue use customized merge logic.
    ///
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

        // Merge exteriors
        for (key, exterior) in self.cells.exteriors {
            let entry = target.cells.exteriors.entry(key);
            if let Entry::Occupied(entry) = entry {
                info!("Merging exterior to master: {:?}", entry.key());
                exterior.merge_into(entry.into_mut());
            } else {
                info!("Copying exterior to master: {:?}", entry.key());
                entry.insert(exterior);
            }
        }

        // Merge interiors
        for (key, interior) in self.cells.interiors {
            let entry = target.cells.interiors.entry(key);
            if let Entry::Occupied(entry) = entry {
                info!("Merging interior to master: {}", entry.key());
                interior.merge_into(entry.into_mut());
            } else {
                info!("Copying interior to master: {}", entry.key());
                entry.insert(interior);
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
        debug_assert!(
            // These types must use their custom merge logic instead.
            !matches!(
                self,
                TES3Object::Header(_)
                    | TES3Object::Cell(_)
                    | TES3Object::Landscape(_)
                    | TES3Object::PathGrid(_)
                    | TES3Object::Dialogue(_)
                    | TES3Object::DialogueInfo(_)
            )
        );
        *target = self;
    }
}

impl MergeInto for Interior {
    fn merge_into(self, target: &mut Self) {
        self.cell.merge_into(&mut target.cell);

        if self.pathgrid.is_some() {
            target.pathgrid = self.pathgrid;
        }
    }
}

impl MergeInto for Exterior {
    fn merge_into(self, target: &mut Self) {
        self.cell.merge_into(&mut target.cell);

        if self.landscape.is_some() {
            target.landscape = self.landscape;
        }
        if self.pathgrid.is_some() {
            target.pathgrid = self.pathgrid;
        }
    }
}

impl MergeInto for Option<Cell> {
    fn merge_into(self, target: &mut Self) {
        if let Some(target) = target {
            if let Some(cell) = self {
                cell.merge_into(target);
            }
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
