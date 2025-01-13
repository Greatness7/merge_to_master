use tes3::esp::*;

use crate::prelude::*;

type Deletions = HashSet<Uncased<'static>>;

pub trait CleanDeletions {
    #[allow(unused_variables)]
    fn clean_deletions(&mut self, deletions: &Deletions) {}
}

impl CleanDeletions for TES3Object {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        delegate! {
            match self {
                inner => inner.clean_deletions(deletions),
            }
        }
    }
}

impl CleanDeletions for Race {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.spells.clean(deletions);
    }
}

impl CleanDeletions for SoundGen {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.creature.clean(deletions);
        // self.sound.clean(deletions);
    }
}

impl CleanDeletions for MagicEffect {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.bolt_sound.clean(deletions);
        self.cast_sound.clean(deletions);
        self.hit_sound.clean(deletions);
        self.area_sound.clean(deletions);
        self.cast_visual.clean(deletions);
        self.bolt_visual.clean(deletions);
        self.hit_visual.clean(deletions);
        self.area_visual.clean(deletions);
    }
}

impl CleanDeletions for Region {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.sleep_creature.clean(deletions);
        self.sounds.clean(deletions);
    }
}

impl CleanDeletions for Birthsign {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.spells.clean(deletions);
    }
}

impl CleanDeletions for StartScript {
    fn clean_deletions(&mut self, _deletions: &Deletions) {
        // self.script.clean(deletions);
    }
}

impl CleanDeletions for Door {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(deletions);
        self.open_sound.clean(deletions);
        self.close_sound.clean(deletions);
    }
}

impl CleanDeletions for MiscItem {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(deletions);
    }
}

impl CleanDeletions for Weapon {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(deletions);
        self.enchanting.clean(deletions);
    }
}

impl CleanDeletions for Container {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(deletions);
        self.inventory.clean(deletions);
    }
}

impl CleanDeletions for Creature {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(deletions);
        self.inventory.clean(deletions);
        self.spells.clean(deletions);
        // self.ai_packages.clean(deletions); // TODO
        // self.travel_destinations.clean(deletions); // TODO
    }
}

impl CleanDeletions for Light {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(deletions);
        self.sound.clean(deletions);
    }
}

impl CleanDeletions for Npc {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(deletions);
        self.inventory.clean(deletions);
        self.spells.clean(deletions);
        // self.ai_packages.clean(deletions); // TODO
        // self.travel_destinations.clean(deletions); // TODO
        // self.race.clean(deletions);
        // self.class.clean(deletions);
        self.faction.clean(deletions);
        // self.head.clean(deletions);
        // self.hair.clean(deletions);
    }
}

impl CleanDeletions for Armor {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(deletions);
        self.enchanting.clean(deletions);
        for object in &mut self.biped_objects {
            object.male_bodypart.clean(deletions);
            object.female_bodypart.clean(deletions);
        }
    }
}

impl CleanDeletions for Clothing {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(deletions);
        self.enchanting.clean(deletions);
        for object in &mut self.biped_objects {
            object.male_bodypart.clean(deletions);
            object.female_bodypart.clean(deletions);
        }
    }
}

impl CleanDeletions for RepairItem {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(deletions);
    }
}

impl CleanDeletions for Activator {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(deletions);
    }
}

impl CleanDeletions for Apparatus {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(deletions);
    }
}

impl CleanDeletions for Lockpick {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(deletions);
    }
}

impl CleanDeletions for Probe {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(deletions);
    }
}

impl CleanDeletions for Ingredient {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(deletions);
    }
}

impl CleanDeletions for Book {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(deletions);
        self.enchanting.clean(deletions);
    }
}

impl CleanDeletions for Alchemy {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(deletions);
    }
}

impl CleanDeletions for LeveledItem {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.items.clean(deletions);
    }
}

impl CleanDeletions for LeveledCreature {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.creatures.clean(deletions);
    }
}

impl CleanDeletions for Cell {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.region.clean(deletions);

        // let cell_name = self.editor_id().into_owned();

        self.references.retain(|_indices, reference| {
            // Retain referances that are not local to the plugin.
            if reference.mast_index != 0 {
                return true;
            }

            // Discard references that are explicitly marked as deleted.
            if reference.deleted == Some(true) {
                return false;
            }

            // Discard references that are implicitly deleted via object.
            if !deletions.is_empty() {
                let id = reference.id.as_uncased();
                if deletions.contains(id) {
                    // info!("Removed deleted reference: {id} {indices:?} from {cell_name}");
                    return false;
                }
            }

            // Retain all non-deleted references.
            true
        });
    }
}

impl CleanDeletions for Header {}
impl CleanDeletions for GameSetting {}
impl CleanDeletions for GlobalVariable {}
impl CleanDeletions for Class {}
impl CleanDeletions for Faction {}
impl CleanDeletions for Sound {}
impl CleanDeletions for Skill {}
impl CleanDeletions for Script {}
impl CleanDeletions for LandscapeTexture {}
impl CleanDeletions for Spell {}
impl CleanDeletions for Static {}
impl CleanDeletions for Bodypart {}
impl CleanDeletions for Enchanting {}
impl CleanDeletions for Landscape {}
impl CleanDeletions for PathGrid {}
impl CleanDeletions for Dialogue {}
impl CleanDeletions for DialogueInfo {}

// ---------------------------------------------------------------------------

use std::borrow::Borrow;

#[ext]
impl String {
    fn clean(&mut self, deletions: &Deletions) {
        if deletions.contains(self.as_uncased()) {
            self.clear();
        }
    }
}

#[ext]
impl Option<String> {
    fn clean(&mut self, deletions: &Deletions) {
        if let Some(mut string) = self.take() {
            string.clean(deletions);
            if !string.is_empty() {
                *self = Some(string);
            }
        }
    }
}

#[ext]
impl<S> Vec<S>
where
    S: Borrow<str>,
{
    fn clean(&mut self, deletions: &Deletions) {
        self.retain(|id| !deletions.contains(id.borrow().as_uncased()));
    }
}

#[ext]
impl<S, T> Vec<(S, T)>
where
    S: Borrow<str>,
{
    fn clean(&mut self, deletions: &Deletions) {
        self.retain(|(id, _)| !deletions.contains(id.borrow().as_uncased()));
    }
}

#[ext]
impl<S, T> Vec<(T, S)>
where
    S: Borrow<str>,
{
    fn clean(&mut self, deletions: &Deletions) {
        self.retain(|(_, id)| !deletions.contains(id.borrow().as_uncased()));
    }
}
