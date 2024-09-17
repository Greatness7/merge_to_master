use std::collections::VecDeque;

use tes3::esp::*;

use crate::prelude::*;

type ObjectId = String;
type Tag = &'static [u8; 4];
type TaggedId = (Tag, ObjectId);

#[derive(Default)]
pub struct PluginData {
    pub header: Header,
    pub objects: HashMap<TaggedId, TES3Object>,
    pub dialogues: HashMap<ObjectId, DialogueGroup>,
}

#[derive(Default)]
pub struct DialogueGroup {
    pub dialogue: Dialogue,
    pub infos: VecDeque<DialogueInfo>,
}

impl PluginData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_path(path: &Path) -> Result<Self> {
        Ok(Self::from_plugin(
            Plugin::from_path(path) //
                .with_context(|| format!("Path: {path:?}"))?,
        ))
    }

    pub fn save_path(self, path: &Path) -> Result<()> {
        self.into_plugin().save_path(path)?;
        Ok(())
    }

    pub fn from_plugin(plugin: Plugin) -> Self {
        let mut this = Self::default();
        this.collect_objects(plugin);
        this
    }

    pub fn into_plugin(mut self) -> Plugin {
        let mut plugin = Plugin::new();
        plugin.objects.reserve(self.num_objects());
        plugin.objects.push(self.header.into());
        plugin.objects.extend(self.objects.into_values());
        plugin.objects.extend(self.dialogues.extract_journals());
        plugin.objects.extend(self.dialogues.into_objects());
        plugin.sort_objects();
        plugin
    }

    pub fn from_path_dialogue_only(path: &Path) -> Result<Self> {
        let mut plugin = Plugin::new();
        plugin
            .load_path_filtered(path, |tag| matches!(&tag, Dialogue::TAG | DialogueInfo::TAG))
            .with_context(|| format!("Path: {path:?}"))?;
        Ok(Self::from_plugin(plugin))
    }

    fn num_objects(&self) -> usize {
        1 /* Header */
            + self.objects.len()
            + self.dialogues.len()
            + self.dialogues.values().map(|group| group.infos.len()).sum::<usize>()
    }

    pub fn next_reference_index(&self) -> u32 {
        self.objects
            .par_values()
            .filter_map(|object| {
                let cell: &Cell = object.try_into().ok()?;
                cell.references
                    .values()
                    .filter_map(|reference| (reference.mast_index == 0).then_some(reference.refr_index))
                    .max()
            })
            .max()
            .map_or(1, |i| i + 1)
    }

    pub fn next_texture_index(&self) -> Option<u32> {
        self.objects
            .par_values()
            .filter_map(|object| {
                let texture: &LandscapeTexture = object.try_into().ok()?;
                Some(texture.index)
            })
            .max()
            .map(|i| i + 1)
    }

    pub fn collect_objects(&mut self, plugin: Plugin) {
        let mut dialogue_id = String::with_capacity(32);

        for object in plugin.objects {
            use TES3Object::*;

            let (tag, id) = match object {
                Header(header) => {
                    self.header = header;
                    continue;
                }
                Dialogue(dialogue) => {
                    dialogue_id.clear();
                    dialogue_id.push_str(&dialogue.editor_id_ascii_lowercase());
                    let group = self.dialogues.entry_ref(&dialogue_id).or_default();
                    group.dialogue = dialogue;
                    continue;
                }
                DialogueInfo(info) => {
                    let group = self.dialogues.get_mut(&dialogue_id).expect("Orphan DialogueInfo");
                    group.insert_info(info);
                    continue;
                }
                _ => {
                    const TAG: Tag = &[0; 4];

                    let tag = object.tag();
                    let id = object.editor_id().to_ascii_lowercase();

                    if matches!(
                        object,
                        Activator(_)
                            | Alchemy(_)
                            | Apparatus(_)
                            | Armor(_)
                            | Bodypart(_)
                            | Book(_)
                            | Clothing(_)
                            | Container(_)
                            | Creature(_)
                            | Door(_)
                            | Enchanting(_)
                            | Ingredient(_)
                            | LeveledCreature(_)
                            | LeveledItem(_)
                            | Light(_)
                            | Lockpick(_)
                            | MiscItem(_)
                            | Npc(_)
                            | Probe(_)
                            | RepairItem(_)
                            | Spell(_)
                            | Static(_)
                            | Weapon(_)
                    ) {
                        (TAG, id)
                    } else {
                        (tag, id)
                    }
                }
            };
            if !id.is_empty() {
                self.objects.insert((tag, id), object);
            }
        }
    }

    pub fn set_all_ignored(&mut self, ignored: bool) {
        for object in self.objects.values_mut() {
            object.set_ignored(ignored);
        }
        for group in self.dialogues.values_mut() {
            group.dialogue.set_ignored(ignored);
            for info in group.infos.iter_mut() {
                info.set_ignored(ignored);
            }
        }
    }

    pub fn remove_ignored(&mut self) {
        self.objects.retain(|_, object| !object.ignored());
        self.dialogues.retain(|_, group| !group.dialogue.ignored());
        for group in self.dialogues.values_mut() {
            group.infos.retain(|info| !info.ignored());
        }
    }

    pub fn remove_deleted(&mut self) {
        let deleted_objects: HashSet<_> = self
            .objects
            .extract_if(|_, object| object.deleted())
            .map(|((_, id), _)| id)
            .collect();

        for id in &deleted_objects {
            info!("Removed deleted object: {id}");
        }

        // discard deleted (local) references
        for ((_, cell_name), object) in self.objects.iter_mut() {
            let TES3Object::Cell(cell) = object else {
                continue;
            };

            cell.references.retain(|indices, reference| {
                // Retain referances that are not local to the plugin.
                if reference.mast_index != 0 {
                    return true;
                }

                // Discard references that are explicitly marked as deleted.
                if reference.deleted == Some(true) {
                    return false;
                }

                // Discard references that are implicitly deleted via object.
                if !deleted_objects.is_empty() {
                    let id = reference.id.to_ascii_lowercase();
                    if deleted_objects.contains(&id) {
                        info!("Removed deleted reference: {id} {indices:?} from {cell_name}");
                        return false;
                    }
                }

                // Retain all non-deleted references.
                true
            });
        }

        // TODO: discard deleted dialogue
        // This might entail rebuilding the prev/next links.
    }
}

impl DialogueGroup {
    /// Finds the index of the `DialogueInfo` with the specified `id`.
    ///
    fn find(&self, id: &str) -> Option<usize> {
        // Searching in reverse is faster because we're often calling find
        // on the `prev_id`, which is usually the last element in the list.
        self.infos
            .iter()
            .rev()
            .position(|info| info.id == id)
            .map(|i| self.infos.len() - 1 - i)
    }

    /// Inserts a new `DialogueInfo` into the `DialogueGroup`.
    ///
    /// If an `INFO` with the same `id` already exists then it will be replaced.
    ///
    pub fn insert_info(&mut self, info: DialogueInfo) {
        // Does an INFO with the this id already exist?
        if let Some(i) = self.find(&info.id) {
            // If the previous `next_id` is already correct do an in-place update.
            // This happens when the text was changed but ordering was unmodified.
            if self.infos[i].prev_id == info.prev_id {
                self.infos[i] = info;
                return;
            }

            // Otherwise it already exists but the ordering has been changed.
            // Delete the old entry so we can insert it in the correct place.
            self.infos.remove(i);
        }

        // If no `prev_id` was specified then insert the INFO at the front of list.
        if info.prev_id.is_empty() {
            self.infos.push_front(info);
            return;
        }

        // If the `prev_id` was specified and already exists, then insert after it.
        if let Some(i) = self.find(&info.prev_id) {
            self.infos.insert(i + 1, info);
            return;
        }

        // A `prev_id` was specified, but not found, insert at the end of the list.
        self.infos.push_back(info);
    }

    /// Repairs the `prev_id` and `next_id` links between `DialogueInfo` objects.
    ///
    /// Note: Both front/back links are left unmodified to match engine behavior.
    ///
    pub fn repair_links(&mut self) {
        use lending_iterator::prelude::*;

        let mut windows = self.infos.make_contiguous().windows_mut();

        while let Some([prev, curr, next]) = windows.next() {
            if prev.next_id != curr.id {
                prev.next_id.clear();
                prev.next_id.push_str(&curr.id);
            }
            if curr.prev_id != prev.id {
                curr.prev_id.clear();
                curr.prev_id.push_str(&prev.id);
            }
            if curr.next_id != next.id {
                curr.next_id.clear();
                curr.next_id.push_str(&next.id);
            }
            if next.prev_id != curr.id {
                next.prev_id.clear();
                next.prev_id.push_str(&curr.id);
            }
        }
    }
}

#[ext]
impl HashMap<ObjectId, DialogueGroup> {
    fn into_objects(self) -> impl IntoIterator<Item = TES3Object> {
        self.into_values().flat_map(|group| {
            std::iter::once(group.dialogue.into()) //
                .chain(group.infos.into_iter().map_into())
        })
    }

    fn extract_journals(&mut self) -> impl IntoIterator<Item = TES3Object> {
        self.extract_if(|_, group| group.dialogue.dialogue_type == DialogueType2::Journal)
            .flat_map(|(_, group)| {
                std::iter::once(group.dialogue.into()) //
                    .chain(group.infos.into_iter().map_into())
            })
    }
}
