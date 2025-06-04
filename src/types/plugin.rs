use tes3::esp::*;

use crate::prelude::*;

#[derive(Default)]
pub struct PluginData {
    pub header: Header,
    pub objects: Objects,
    pub cells: Cells,
    pub dialogues: Dialogues,
}

impl PluginData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_path(path: &Path) -> Result<Self> {
        Ok(Self::from_plugin(
            Plugin::from_path(path) //
                .with_context(|| path.display().to_string())?,
        ))
    }

    pub fn save_path(self, path: &Path) -> Result<()> {
        self.into_plugin()
            .save_path(path)
            .with_context(|| path.display().to_string())
    }

    pub fn from_plugin(plugin: Plugin) -> Self {
        let mut this = Self::default();
        this.collect_objects(plugin);
        this
    }

    pub fn into_plugin(self) -> Plugin {
        let mut plugin = Plugin::new();
        plugin.objects.reserve(self.count_objects());
        plugin.objects.push(self.header.into());
        plugin.objects.extend(self.objects.into_values());
        plugin.objects.extend(self.cells.into_objects());
        plugin.objects.extend(self.dialogues.into_objects());
        plugin.sort_objects();
        plugin
    }

    pub(crate) fn from_path_partial(path: &Path) -> Result<Self> {
        let mut plugin = Plugin::from_path_filtered(path, |tag| {
            matches!(&tag, Cell::TAG | Dialogue::TAG | DialogueInfo::TAG)
        })
        .with_context(|| path.display().to_string())?;

        // We only need certain attributes, discard any unnecessary stuff.
        for cell in plugin.objects_of_type_mut() {
            let Cell { flags, name, data, .. } = std::mem::take(cell);
            cell.flags = flags;
            cell.name = name;
            cell.data = data;
        }

        Ok(Self::from_plugin(plugin))
    }

    #[rustfmt::skip]
    fn collect_objects(&mut self, plugin: Plugin) {
        let mut dialogue_id = String::with_capacity(32);

        // TODO: What happens if there is a non-INFO object threaded within
        //       the dialogue info list? Does that break the topic grouping?

        for object in plugin.objects {
            use TES3Object::*;

            match object {
                Header(header) => {
                    self.header = header;
                }

                // ---------------------------------------------------------------------------

                // Uses (tag, id) as our key so different object types can share the same id.

                Birthsign(_)
                | Class(_)
                | Faction(_)
                | GlobalVariable(_)
                | LandscapeTexture(_)
                | MagicEffect(_)
                | Race(_)
                | Region(_)
                | Script(_)
                | Skill(_)
                | Sound(_)
                | SoundGen(_)
                | StartScript(_)
                | GameSetting(_)
                => {
                    let tag = object.tag();
                    let id = object.editor_id().to_ascii_lowercase();
                    if !id.is_empty() {
                        self.objects.insert((tag, id), object);
                    }
                }

                // For "physical" objects the id should be unique amongst all other physical
                // objects in the world. We replicate this behavior with a fixed tag as key.

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
                | Enchanting(_) // Not a PhysicalObject, but uses same ID rules?
                | Ingredient(_)
                | LeveledCreature(_)
                | LeveledItem(_)
                | Light(_)
                | Lockpick(_)
                | MiscItem(_)
                | Npc(_)
                | Probe(_)
                | RepairItem(_)
                | Spell(_) // Not a PhysicalObject, but uses same ID rules?
                | Static(_)
                | Weapon(_)
                => {
                    let tag = &[0; 4];
                    let id = object.editor_id().to_ascii_lowercase();
                    if !id.is_empty() {
                        self.objects.insert((tag, id), object);
                    }
                }


                // ---------------------------------------------------------------------------

                Cell(cell) => {
                    if let Some(coords) = cell.exterior_coords() {
                        let exterior = self.cells.get_or_create_exterior(coords);
                        exterior.cell = Some(cell);
                    } else {
                        let interior = self.cells.get_or_create_interior(&cell.name);
                        interior.cell = Some(cell);
                    }
                }
                Landscape(landscape) => {
                    let exterior = self.cells.get_or_create_exterior(landscape.grid);
                    exterior.landscape = Some(landscape);
                }
                PathGrid(pathgrid) => {
                    if let Some(interior) = self.cells.get_interior_mut(&pathgrid.cell) {
                        interior.pathgrid = Some(pathgrid);
                    } else if let Some(exterior) = self.cells.get_exterior_mut(pathgrid.data.grid) {
                        exterior.pathgrid = Some(pathgrid);
                    } else {
                        // An awkward situation where the path grid belongs to a cell that is not
                        // present in the plugin. This happens when a tool like `tes3cmd clean`
                        // has incorrectly "cleaned" the associated cell entry.
                        warn!("Orphan PathGrid: {}", pathgrid.editor_id());

                        // Path grids for interiors default to (0, 0) as their grid coordinates.
                        //
                        // This means when we encounter a path grid of an invalid cell there are two
                        // interpretations:
                        //
                        // 1) The cell is valid, but is defined somewhere prior in the load order.
                        // 2) The path grid actually *does* belong to the (0, 0) exterior.
                        //
                        // Case (1) is likely much more common, few mods edit the origin exterior, so
                        // we will go ahead and assume that to be the intention here.
                        //
                        // Note that the TESCS also handles this case poorly and will end up applying
                        // this path grid to the (0, 0) exterior. Though with different semantics as
                        // it executes plugin loading linearly thus can see cells from prior plugins.
                        //
                        if pathgrid.data.grid == (0, 0) && !pathgrid.cell.is_empty() {
                            let interior = self.cells.get_or_create_interior(&pathgrid.cell);
                            interior.pathgrid = Some(pathgrid);
                        } else {
                            let exterior = self.cells.get_or_create_exterior(pathgrid.data.grid);
                            exterior.pathgrid = Some(pathgrid);
                        }
                    }
                }

                // ---------------------------------------------------------------------------

                Dialogue(dialogue) => {
                    dialogue_id.clear();
                    dialogue_id.push_str(&dialogue.id);
                    dialogue_id.make_ascii_lowercase();
                    let group = self.dialogues.entry_ref(&dialogue_id).or_default();
                    group.dialogue = dialogue;
                }
                DialogueInfo(info) => {
                    let group = self.dialogues.get_mut(&dialogue_id).expect("Orphan DialogueInfo");
                    group.insert_info(info);
                }
            }
        }
    }

    pub(crate) fn set_all_ignored(&mut self, ignored: bool) {
        for object in self.objects.values_mut() {
            object.set_ignored(ignored);
        }
        for interior in self.cells.interiors.values_mut() {
            interior.cell.set_ignored(ignored);
            interior.pathgrid.set_ignored(ignored);
        }
        for exterior in self.cells.exteriors.values_mut() {
            exterior.cell.set_ignored(ignored);
            exterior.landscape.set_ignored(ignored);
            exterior.pathgrid.set_ignored(ignored);
        }
        for group in self.dialogues.values_mut() {
            group.dialogue.set_ignored(ignored);
            for info in group.infos.iter_mut() {
                info.set_ignored(ignored);
            }
        }
    }
}
