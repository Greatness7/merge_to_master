use tes3::esp::*;

use crate::prelude::*;

pub trait RemoveDeleted {
    /// Remove all objects that are marked as deleted.
    ///
    fn remove_deleted(&mut self);
}

impl RemoveDeleted for PluginData {
    fn remove_deleted(&mut self) {
        let mut deletions = Deletions::new();

        self.objects
            .extract_if(|_, object| object.deleted())
            .for_each(|((_, id), object)| {
                info!("Removed deleted {} object: {}", object.tag_str(), object.editor_id());
                deletions.entry(id.into()).or_default().insert(object.into());
            });

        self.cells
            .interiors
            .extract_if(|_, interior| interior.cell.as_ref().is_some_and(<_>::deleted))
            .for_each(|(id, _)| {
                info!("Removed deleted interior: {}", id.as_str());
                deletions.entry(id).or_default().insert(DeletionFlags::CELL);
            });

        self.objects //
            .par_values_mut()
            .for_each(|object| {
                object.clean_deletions(&deletions);
            });

        // Note: We still need to run this code even if `deletions` is empty.
        // Because it also takes care of cleaning up deleted cell references.
        self.cells //
            .par_iter_mut()
            .for_each(|cell| {
                cell.clean_deletions(&deletions);
            });

        self.cells.remove_deleted();
        self.dialogues.remove_deleted();
    }
}

impl RemoveDeleted for Cells {
    fn remove_deleted(&mut self) {
        self.exteriors.remove_deleted();
        self.interiors.remove_deleted();
    }
}

impl RemoveDeleted for HashMap<(i32, i32), Exterior> {
    fn remove_deleted(&mut self) {
        self.retain(|id, exterior| {
            if exterior.cell.discard_deleted() {
                info!("Removed deleted exterior: {id:?}");
            }
            if exterior.pathgrid.discard_deleted() {
                info!("Removed deleted exterior pathgrid: {id:?}");
            }
            if exterior.landscape.discard_deleted() {
                info!("Removed deleted exterior landscape: {id:?}");
            }
            exterior.cell.is_some() && exterior.count_objects() != 0
        });
    }
}

impl RemoveDeleted for HashMap<UString, Interior> {
    fn remove_deleted(&mut self) {
        self.retain(|id, interior| {
            if interior.cell.discard_deleted() {
                info!("Removed deleted interior: {id}");
            }
            if interior.pathgrid.discard_deleted() {
                info!("Removed deleted interior pathgrid: {id}");
            }
            interior.cell.is_some() && interior.count_objects() != 0
        });
    }
}

impl RemoveDeleted for HashMap<String, DialogueGroup> {
    fn remove_deleted(&mut self) {
        self.retain(|_, group| !group.dialogue.deleted());

        for group in self.values_mut() {
            if !group.infos.iter().any(<_>::deleted) {
                continue;
            }

            // If the ends were deleted we need to clear the links later.
            let (front_deleted, back_deleted) = (
                group.infos.front().is_some_and(<_>::deleted),
                group.infos.back().is_some_and(<_>::deleted),
            );

            group.infos.retain(|info| !info.deleted());
            if !group.infos.is_empty() {
                group.repair_links();

                if front_deleted {
                    group.infos.front_mut().unwrap().prev_id.clear();
                }
                if back_deleted {
                    group.infos.back_mut().unwrap().next_id.clear();
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------

/// Maps object ids of deleted objects to their types.
///
type Deletions = HashMap<UString, DeletionFlags>;

trait CleanDeletions {
    /// Cleans all fields that refer to deleted objects.
    ///
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
        self.spells.clean(DeletionFlags::SPELL, deletions);
    }
}

impl CleanDeletions for SoundGen {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.creature.clean(DeletionFlags::PHYSICAL, deletions);
        self.sound.clean(DeletionFlags::SOUND, deletions);
    }
}

impl CleanDeletions for MagicEffect {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.bolt_sound.clean(DeletionFlags::SOUND, deletions);
        self.cast_sound.clean(DeletionFlags::SOUND, deletions);
        self.hit_sound.clean(DeletionFlags::SOUND, deletions);
        self.area_sound.clean(DeletionFlags::SOUND, deletions);
        self.cast_visual.clean(DeletionFlags::PHYSICAL, deletions);
        self.bolt_visual.clean(DeletionFlags::PHYSICAL, deletions);
        self.hit_visual.clean(DeletionFlags::PHYSICAL, deletions);
        self.area_visual.clean(DeletionFlags::PHYSICAL, deletions);
    }
}

impl CleanDeletions for Region {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.sleep_creature.clean(DeletionFlags::PHYSICAL, deletions);
        self.sounds.clean(DeletionFlags::SOUND, deletions);
    }
}

impl CleanDeletions for Birthsign {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.spells.clean(DeletionFlags::SPELL, deletions);
    }
}

impl CleanDeletions for StartScript {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(DeletionFlags::SCRIPT, deletions);
    }
}

impl CleanDeletions for Door {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(DeletionFlags::SCRIPT, deletions);
        self.open_sound.clean(DeletionFlags::SOUND, deletions);
        self.close_sound.clean(DeletionFlags::SOUND, deletions);
    }
}

impl CleanDeletions for MiscItem {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(DeletionFlags::SCRIPT, deletions);
    }
}

impl CleanDeletions for Weapon {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(DeletionFlags::SCRIPT, deletions);
        self.enchanting.clean(DeletionFlags::ENCHANTING, deletions);
    }
}

impl CleanDeletions for Container {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(DeletionFlags::SCRIPT, deletions);
        self.inventory.clean(DeletionFlags::PHYSICAL, deletions);
    }
}

impl CleanDeletions for Creature {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(DeletionFlags::SCRIPT, deletions);
        self.inventory.clean(DeletionFlags::PHYSICAL, deletions);
        self.spells.clean(DeletionFlags::SPELL, deletions);
        self.ai_packages.clean_deletions(deletions);
        self.travel_destinations.clean_deletions(deletions);
    }
}

impl CleanDeletions for Light {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(DeletionFlags::SCRIPT, deletions);
        self.sound.clean(DeletionFlags::SOUND, deletions);
    }
}

impl CleanDeletions for Npc {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(DeletionFlags::SCRIPT, deletions);
        self.inventory.clean(DeletionFlags::PHYSICAL, deletions);
        self.spells.clean(DeletionFlags::SPELL, deletions);
        self.ai_packages.clean_deletions(deletions);
        self.travel_destinations.clean_deletions(deletions);
        // self.race.clean(DeletionType::RACE, deletions); // Crashes TESCS if cleaned.
        self.class.clean(DeletionFlags::CLASS, deletions);
        self.faction.clean(DeletionFlags::FACTION, deletions);
        self.head.clean(DeletionFlags::PHYSICAL, deletions);
        self.hair.clean(DeletionFlags::PHYSICAL, deletions);
    }
}

impl CleanDeletions for Armor {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(DeletionFlags::SCRIPT, deletions);
        self.enchanting.clean(DeletionFlags::ENCHANTING, deletions);
        self.biped_objects.clean_deletions(deletions);
    }
}

impl CleanDeletions for Clothing {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(DeletionFlags::SCRIPT, deletions);
        self.enchanting.clean(DeletionFlags::ENCHANTING, deletions);
        self.biped_objects.clean_deletions(deletions);
    }
}

impl CleanDeletions for RepairItem {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(DeletionFlags::SCRIPT, deletions);
    }
}

impl CleanDeletions for Activator {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(DeletionFlags::SCRIPT, deletions);
    }
}

impl CleanDeletions for Apparatus {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(DeletionFlags::SCRIPT, deletions);
    }
}

impl CleanDeletions for Lockpick {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(DeletionFlags::SCRIPT, deletions);
    }
}

impl CleanDeletions for Probe {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(DeletionFlags::SCRIPT, deletions);
    }
}

impl CleanDeletions for Ingredient {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(DeletionFlags::SCRIPT, deletions);
    }
}

impl CleanDeletions for Book {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(DeletionFlags::SCRIPT, deletions);
        self.enchanting.clean(DeletionFlags::ENCHANTING, deletions);
    }
}

impl CleanDeletions for Alchemy {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.script.clean(DeletionFlags::SCRIPT, deletions);
    }
}

impl CleanDeletions for LeveledItem {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.items.clean(DeletionFlags::PHYSICAL, deletions);
    }
}

impl CleanDeletions for LeveledCreature {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.creatures.clean(DeletionFlags::PHYSICAL, deletions);
    }
}

impl CleanDeletions for Cell {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.region.clean(DeletionFlags::REGION, deletions);

        // let cell_name = self.editor_id().into_owned();

        self.references.retain(|indices, reference| {
            // Retain referances that are not local to the plugin.
            if reference.mast_index != 0 {
                return true;
            }

            // Discard references that are explicitly marked as deleted.
            if reference.deleted() {
                return false;
            }

            // Discard references that are implicitly deleted via object.
            if deletions.intersects(&reference.id, DeletionFlags::PHYSICAL) {
                info!("Removed deleted reference: {} {:?}", reference.id, indices);
                return false;
            }

            // Retain all non-deleted references.
            true
        });
    }
}

impl CleanDeletions for BipedObject {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.male_bodypart.clean(DeletionFlags::PHYSICAL, deletions);
        self.female_bodypart.clean(DeletionFlags::PHYSICAL, deletions);
    }
}

impl CleanDeletions for TravelDestination {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.cell.clean(DeletionFlags::CELL, deletions);
    }
}

impl CleanDeletions for AiEscortPackage {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.target.clean(DeletionFlags::PHYSICAL, deletions);
        self.cell.clean(DeletionFlags::CELL, deletions);
    }
}

impl CleanDeletions for AiFollowPackage {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.target.clean(DeletionFlags::PHYSICAL, deletions);
        self.cell.clean(DeletionFlags::CELL, deletions);
    }
}

impl CleanDeletions for AiActivatePackage {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        self.target.clean(DeletionFlags::PHYSICAL, deletions);
    }
}

impl CleanDeletions for AiPackage {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        match self {
            AiPackage::Travel(package) => package.clean_deletions(deletions),
            AiPackage::Wander(package) => package.clean_deletions(deletions),
            AiPackage::Escort(package) => package.clean_deletions(deletions),
            AiPackage::Follow(package) => package.clean_deletions(deletions),
            AiPackage::Activate(package) => package.clean_deletions(deletions),
        }
    }
}

impl<T: CleanDeletions> CleanDeletions for Vec<T> {
    fn clean_deletions(&mut self, deletions: &Deletions) {
        for item in self {
            item.clean_deletions(deletions);
        }
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
impl CleanDeletions for AiTravelPackage {}
impl CleanDeletions for AiWanderPackage {}

// ---------------------------------------------------------------------------

bitflags::bitflags! {
    #[derive(Clone, Copy, Default)]
    struct DeletionFlags: u64 {
        const HEADER            = 1 << 0;
        const GAME_SETTING      = 1 << 1;
        const GLOBAL_VARIABLE   = 1 << 2;
        const CLASS             = 1 << 3;
        const FACTION           = 1 << 4;
        const RACE              = 1 << 5;
        const SOUND             = 1 << 6;
        const SOUND_GEN         = 1 << 7;
        const SKILL             = 1 << 8;
        const MAGIC_EFFECT      = 1 << 9;
        const SCRIPT            = 1 << 10;
        const REGION            = 1 << 11;
        const BIRTHSIGN         = 1 << 12;
        const START_SCRIPT      = 1 << 13;
        const LANDSCAPE_TEXTURE = 1 << 14;
        const SPELL             = 1 << 15;
        const STATIC            = 1 << 16;
        const DOOR              = 1 << 17;
        const MISC_ITEM         = 1 << 18;
        const WEAPON            = 1 << 19;
        const CONTAINER         = 1 << 20;
        const CREATURE          = 1 << 21;
        const BODYPART          = 1 << 22;
        const LIGHT             = 1 << 23;
        const ENCHANTING        = 1 << 24;
        const NPC               = 1 << 25;
        const ARMOR             = 1 << 26;
        const CLOTHING          = 1 << 27;
        const REPAIR_ITEM       = 1 << 28;
        const ACTIVATOR         = 1 << 29;
        const APPARATUS         = 1 << 30;
        const LOCKPICK          = 1 << 31;
        const PROBE             = 1 << 32;
        const INGREDIENT        = 1 << 33;
        const BOOK              = 1 << 34;
        const ALCHEMY           = 1 << 35;
        const LEVELED_ITEM      = 1 << 36;
        const LEVELED_CREATURE  = 1 << 37;
        const CELL              = 1 << 38;
        const LANDSCAPE         = 1 << 39;
        const PATH_GRID         = 1 << 40;
        const DIALOGUE          = 1 << 41;
        const DIALOGUE_INFO     = 1 << 42;

        const PHYSICAL = (
            DeletionFlags::ACTIVATOR.bits()
            | DeletionFlags::ALCHEMY.bits()
            | DeletionFlags::APPARATUS.bits()
            | DeletionFlags::ARMOR.bits()
            | DeletionFlags::BODYPART.bits()
            | DeletionFlags::BOOK.bits()
            | DeletionFlags::CLOTHING.bits()
            | DeletionFlags::CONTAINER.bits()
            | DeletionFlags::CREATURE.bits()
            | DeletionFlags::DOOR.bits()
            | DeletionFlags::INGREDIENT.bits()
            | DeletionFlags::LEVELED_CREATURE.bits()
            | DeletionFlags::LEVELED_ITEM.bits()
            | DeletionFlags::LIGHT.bits()
            | DeletionFlags::LOCKPICK.bits()
            | DeletionFlags::MISC_ITEM.bits()
            | DeletionFlags::NPC.bits()
            | DeletionFlags::PROBE.bits()
            | DeletionFlags::REPAIR_ITEM.bits()
            | DeletionFlags::STATIC.bits()
            | DeletionFlags::WEAPON.bits()
        );
    }
}

impl From<TES3Object> for DeletionFlags {
    fn from(object: TES3Object) -> Self {
        match object {
            TES3Object::Header(_) => DeletionFlags::HEADER,
            TES3Object::GameSetting(_) => DeletionFlags::GAME_SETTING,
            TES3Object::GlobalVariable(_) => DeletionFlags::GLOBAL_VARIABLE,
            TES3Object::Class(_) => DeletionFlags::CLASS,
            TES3Object::Faction(_) => DeletionFlags::FACTION,
            TES3Object::Race(_) => DeletionFlags::RACE,
            TES3Object::Sound(_) => DeletionFlags::SOUND,
            TES3Object::SoundGen(_) => DeletionFlags::SOUND_GEN,
            TES3Object::Skill(_) => DeletionFlags::SKILL,
            TES3Object::MagicEffect(_) => DeletionFlags::MAGIC_EFFECT,
            TES3Object::Script(_) => DeletionFlags::SCRIPT,
            TES3Object::Region(_) => DeletionFlags::REGION,
            TES3Object::Birthsign(_) => DeletionFlags::BIRTHSIGN,
            TES3Object::StartScript(_) => DeletionFlags::START_SCRIPT,
            TES3Object::LandscapeTexture(_) => DeletionFlags::LANDSCAPE_TEXTURE,
            TES3Object::Spell(_) => DeletionFlags::SPELL,
            TES3Object::Static(_) => DeletionFlags::STATIC,
            TES3Object::Door(_) => DeletionFlags::DOOR,
            TES3Object::MiscItem(_) => DeletionFlags::MISC_ITEM,
            TES3Object::Weapon(_) => DeletionFlags::WEAPON,
            TES3Object::Container(_) => DeletionFlags::CONTAINER,
            TES3Object::Creature(_) => DeletionFlags::CREATURE,
            TES3Object::Bodypart(_) => DeletionFlags::BODYPART,
            TES3Object::Light(_) => DeletionFlags::LIGHT,
            TES3Object::Enchanting(_) => DeletionFlags::ENCHANTING,
            TES3Object::Npc(_) => DeletionFlags::NPC,
            TES3Object::Armor(_) => DeletionFlags::ARMOR,
            TES3Object::Clothing(_) => DeletionFlags::CLOTHING,
            TES3Object::RepairItem(_) => DeletionFlags::REPAIR_ITEM,
            TES3Object::Activator(_) => DeletionFlags::ACTIVATOR,
            TES3Object::Apparatus(_) => DeletionFlags::APPARATUS,
            TES3Object::Lockpick(_) => DeletionFlags::LOCKPICK,
            TES3Object::Probe(_) => DeletionFlags::PROBE,
            TES3Object::Ingredient(_) => DeletionFlags::INGREDIENT,
            TES3Object::Book(_) => DeletionFlags::BOOK,
            TES3Object::Alchemy(_) => DeletionFlags::ALCHEMY,
            TES3Object::LeveledItem(_) => DeletionFlags::LEVELED_ITEM,
            TES3Object::LeveledCreature(_) => DeletionFlags::LEVELED_CREATURE,
            TES3Object::Cell(_) => DeletionFlags::CELL,
            TES3Object::Landscape(_) => DeletionFlags::LANDSCAPE,
            TES3Object::PathGrid(_) => DeletionFlags::PATH_GRID,
            TES3Object::Dialogue(_) => DeletionFlags::DIALOGUE,
            TES3Object::DialogueInfo(_) => DeletionFlags::DIALOGUE_INFO,
        }
    }
}

// ---------------------------------------------------------------------------

use std::borrow::Borrow;

#[ext]
impl String {
    fn clean(&mut self, flags: DeletionFlags, deletions: &Deletions) {
        if deletions.intersects(self, flags) {
            self.clear();
        }
    }
}

#[ext]
impl Option<String> {
    fn clean(&mut self, flags: DeletionFlags, deletions: &Deletions) {
        if let Some(id) = self.as_ref()
            && deletions.intersects(id, flags)
        {
            *self = None;
        }
    }
}

#[ext]
impl<S> Vec<S>
where
    S: Borrow<str>,
{
    fn clean(&mut self, flags: DeletionFlags, deletions: &Deletions) {
        self.retain(|id| !deletions.intersects(id.borrow(), flags));
    }
}

#[ext]
impl<S, T> Vec<(S, T)>
where
    S: Borrow<str>,
{
    fn clean(&mut self, flags: DeletionFlags, deletions: &Deletions) {
        self.retain(|(id, _)| !deletions.intersects(id.borrow(), flags));
    }
}

#[ext]
impl<S, T> Vec<(T, S)>
where
    S: Borrow<str>,
{
    fn clean(&mut self, flags: DeletionFlags, deletions: &Deletions) {
        self.retain(|(_, id)| !deletions.intersects(id.borrow(), flags));
    }
}

// ---------------------------------------------------------------------------

#[ext]
impl Deletions {
    fn intersects(&self, id: &str, flags: DeletionFlags) -> bool {
        self.get(id.as_uncased()) //
            .is_some_and(|deletion| deletion.intersects(flags))
    }
}

#[ext]
impl<T> Option<T>
where
    T: ObjectInfo,
{
    fn discard_deleted(&mut self) -> bool {
        self.take_if(|value| value.deleted()).is_some()
    }
}
