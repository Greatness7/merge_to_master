use std::collections::VecDeque;

use tes3::esp::{Dialogue, DialogueInfo, TES3Object};

use crate::prelude::*;

pub type Dialogues = HashMap<ObjectId, DialogueGroup>;

#[derive(Default)]
pub struct DialogueGroup {
    pub dialogue: Dialogue,
    pub infos: VecDeque<DialogueInfo>,
}

impl DialogueGroup {
    /// Finds the index of the `DialogueInfo` with the specified `id`.
    ///
    fn find(&self, id: &str) -> Option<usize> {
        // Searching in reverse is faster because we're often calling find
        // on the `prev_id`, which is usually the last element in the list.
        self.infos.iter().rposition(|info| info.id == id)
    }

    /// Inserts a new `DialogueInfo`.
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
