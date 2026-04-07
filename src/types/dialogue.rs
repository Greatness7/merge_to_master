use std::collections::VecDeque;

use tes3::esp::{Dialogue, DialogueInfo, TES3Object};

use crate::prelude::*;

pub type Dialogues = HashMap<ObjectId, DialogueGroup>;

#[derive(Debug, Default)]
pub struct DialogueGroup {
    pub dialogue: Dialogue,
    pub infos: VecDeque<DialogueInfo>,
}

impl DialogueGroup {
    /// Inserts a new `DialogueInfo`.
    ///
    /// If an `INFO` with the same `id` already exists then it will be replaced.
    ///
    pub fn insert_info(&mut self, info: DialogueInfo) {
        self.insert_info_impl(info, &mut ());
    }

    /// Efficiently batch-inserts multiple `DialogueInfo` objects.
    ///
    pub fn merge_infos(&mut self, incoming: impl IntoIterator<Item = DialogueInfo>) {
        let mut index: HashMap<_, _> = self
            .infos
            .iter()
            .enumerate()
            .map(|(i, info)| (info.id.clone(), i))
            .collect();
        for info in incoming {
            self.insert_info_impl(info, &mut index);
        }
    }

    /// Core insertion logic. The `index` provides lookups and tracks positions.
    ///
    fn insert_info_impl(&mut self, info: DialogueInfo, index: &mut impl InfoIndex) {
        // Does an INFO with this id already exist?
        if let Some(i) = index.find(&self.infos, &info.id) {
            // If the previous `prev_id` is already correct do an in-place update.
            // This happens when the text was changed but ordering was unmodified.
            if self.infos[i].prev_id == info.prev_id {
                self.infos[i] = info;
                return;
            }

            // Otherwise it already exists but the ordering has been changed.
            // Delete the old entry so we can insert it in the correct place.
            self.infos.remove(i);
            index.remove(&info.id, i);
        }

        // If no `prev_id` was specified then insert at the front of the list.
        if info.prev_id.is_empty() {
            index.insert(&info.id, 0);
            self.infos.push_front(info);
            return;
        }

        // If the `prev_id` was specified and already exists, insert after it.
        if let Some(i) = index.find(&self.infos, &info.prev_id) {
            index.insert(&info.id, i + 1);
            self.infos.insert(i + 1, info);
            return;
        }

        // A `prev_id` was specified, but not found, insert at the end.
        index.insert(&info.id, self.infos.len());
        self.infos.push_back(info);
    }

    /// Repairs the `prev_id` and `next_id` links between `DialogueInfo` objects.
    ///
    /// Note: Both front/back links are left unmodified to match engine behavior.
    ///
    pub fn repair_links(&mut self) {
        let mut windows = self.infos.make_contiguous().windows_mut();

        while let Some([prev, curr]) = windows.next() {
            if prev.next_id != curr.id {
                prev.next_id.clear();
                prev.next_id.push_str(&curr.id);
            }
            if curr.prev_id != prev.id {
                curr.prev_id.clear();
                curr.prev_id.push_str(&prev.id);
            }
        }
    }
}

trait InfoIndex {
    #[inline]
    fn find(&self, infos: &VecDeque<DialogueInfo>, id: &str) -> Option<usize> {
        infos.iter().rposition(|info| info.id == id)
    }
    #[inline]
    fn insert(&mut self, _id: &str, _pos: usize) {}
    #[inline]
    fn remove(&mut self, _id: &str, _pos: usize) {}
}

impl InfoIndex for () {}

impl InfoIndex for HashMap<String, usize> {
    #[inline]
    fn find(&self, _infos: &VecDeque<DialogueInfo>, id: &str) -> Option<usize> {
        self.get(id).copied()
    }
    #[inline]
    fn insert(&mut self, id: &str, pos: usize) {
        for value in self.values_mut() {
            if *value >= pos {
                *value += 1;
            }
        }
        self.insert(id.into(), pos);
    }
    #[inline]
    fn remove(&mut self, id: &str, pos: usize) {
        self.remove(id);
        for value in self.values_mut() {
            if *value > pos {
                *value -= 1;
            }
        }
    }
}

#[ext]
#[doc(hidden)]
impl HashMap<ObjectId, DialogueGroup> {
    pub fn into_objects(self) -> impl IntoIterator<Item = TES3Object> {
        let priority = |dialogue: &Dialogue| {
            use tes3::esp::DialogueType2::*;
            match dialogue.dialogue_type {
                Journal => 0, // Note: It is *required* that journals are sorted first!
                Topic => 1,
                Voice => 2,
                Greeting => 3,
                Persuasion => 4,
            }
        };
        self.into_values()
            .sorted_by(|a, b| {
                let p1 = priority(&a.dialogue);
                let p2 = priority(&b.dialogue);
                Ord::cmp(&p1, &p2) //
                    .then_with(|| a.dialogue.id.cmp(&b.dialogue.id))
            })
            .flat_map(|group| {
                itertools::chain(
                    Some(group.dialogue.into()),        //
                    group.infos.into_iter().map_into(), //
                )
            })
    }
}
