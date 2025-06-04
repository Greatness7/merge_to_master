use tes3::esp::TES3Object;

use crate::prelude::*;

pub trait IntoObjects {
    fn into_objects(self) -> impl IntoIterator<Item = TES3Object>;
}

impl IntoObjects for Exterior {
    fn into_objects(self) -> impl IntoIterator<Item = TES3Object> {
        itertools::chain!(
            self.cell.map_into(), //
            self.landscape.map_into(),
            self.pathgrid.map_into()
        )
    }
}

impl IntoObjects for Interior {
    fn into_objects(self) -> impl IntoIterator<Item = TES3Object> {
        itertools::chain!(
            self.cell.map_into(), //
            self.pathgrid.map_into()
        )
    }
}

impl IntoObjects for Cells {
    fn into_objects(self) -> impl IntoIterator<Item = TES3Object> {
        itertools::chain!(
            self.exteriors.into_objects(), //
            self.interiors.into_objects()
        )
    }
}

impl<K> IntoObjects for HashMap<K, TES3Object>
where
    K: Clone + Ord + std::hash::Hash,
{
    fn into_objects(self) -> impl IntoIterator<Item = TES3Object> {
        self.into_values()
    }
}

impl<K, V> IntoObjects for HashMap<K, V>
where
    K: Clone + Ord + std::hash::Hash,
    V: IntoObjects,
{
    fn into_objects(mut self) -> impl IntoIterator<Item = TES3Object> {
        self.keys()
            .cloned()
            .sorted()
            .filter_map(move |key| self.remove(&key))
            .flat_map(IntoObjects::into_objects)
    }
}
