use tes3::esp::TES3Object;

use crate::prelude::*;

pub trait CountObjects {
    /// The number of contained `TES3Object` instances.
    ///
    fn count_objects(&self) -> usize;
}

impl CountObjects for PluginData {
    fn count_objects(&self) -> usize {
        1 /* Header */
            + self.objects.count_objects()
            + self.cells.count_objects()
            + self.dialogues.count_objects()
    }
}

impl CountObjects for Cells {
    fn count_objects(&self) -> usize {
        self.interiors.count_objects() + self.exteriors.count_objects()
    }
}

impl CountObjects for Interior {
    fn count_objects(&self) -> usize {
        self.cell.count_objects() + self.pathgrid.count_objects()
    }
}

impl CountObjects for Exterior {
    fn count_objects(&self) -> usize {
        self.cell.count_objects() + self.landscape.count_objects() + self.pathgrid.count_objects()
    }
}

impl CountObjects for DialogueGroup {
    fn count_objects(&self) -> usize {
        1 /* Dialogue */ + self.infos.len()
    }
}

impl<K, V> CountObjects for HashMap<K, V>
where
    V: CountObjects,
{
    fn count_objects(&self) -> usize {
        self.values().map(V::count_objects).sum()
    }
}

impl<T> CountObjects for Option<T>
where
    T: Into<TES3Object>,
{
    fn count_objects(&self) -> usize {
        self.is_some().into()
    }
}

impl CountObjects for Vec<TES3Object> {
    fn count_objects(&self) -> usize {
        self.len()
    }
}

impl<K> CountObjects for HashMap<K, TES3Object> {
    fn count_objects(&self) -> usize {
        self.len()
    }
}
