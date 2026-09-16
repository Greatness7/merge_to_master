use std::collections::VecDeque;
use std::hash::BuildHasher;

use hashbrown::{DefaultHashBuilder, HashTable, hash_table};
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
    /// The `index` must be created from this group's `infos`, and be used for
    /// all insertions into this group until it is discarded or reset.
    ///
    pub fn insert_info(&mut self, info: DialogueInfo, index: &mut InfoIndex) {
        // Does an INFO with this id already exist? New ids don't need a search.
        if !index.insert(&info.id)
            && let Some(i) = InfoIndex::find(&self.infos, &info.id, index.last + 1)
        {
            // If the previous `prev_id` is already correct do an in-place update.
            // This happens when the text was changed but ordering was unmodified.
            if self.infos[i].prev_id == info.prev_id {
                self.infos[i] = info;
                index.last = i;
                return;
            }

            // Otherwise it already exists but the ordering has been changed.
            // Delete the old entry so we can insert it in the correct place.
            self.infos.remove(i);
        }

        // If no `prev_id` was specified then insert at the front of the list.
        if info.prev_id.is_empty() {
            self.infos.push_front(info);
            index.last = 0;
            return;
        }

        // If the `prev_id` was specified and already exists, insert after it.
        if index.contains(&info.prev_id)
            && let Some(i) = InfoIndex::find(&self.infos, &info.prev_id, index.last)
        {
            self.infos.insert(i + 1, info);
            index.last = i + 1;
            return;
        }

        // A `prev_id` was specified, but not found, insert at the end.
        self.infos.push_back(info);
        index.last = self.infos.len() - 1;
    }

    /// Efficiently batch-inserts multiple `DialogueInfo` objects.
    ///
    pub fn merge_infos(&mut self, incoming: impl IntoIterator<Item = DialogueInfo>) {
        let mut index = InfoIndex::new(&self.infos);
        for info in incoming {
            self.insert_info(info, &mut index);
        }
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

/// Speeds up `DialogueGroup::insert_info` lookups.
///
/// Plugins usually store INFOs in runs that follow their linked list order, so
/// the next INFO's `prev_id` is typically the INFO that was inserted last. The
/// `last` position is only a hint, and is verified before being used.
///
/// The hash of every id in the group is stored, so searches for new ids can be
/// skipped. Only hashes are stored, which avoids cloning the id strings. A hash
/// collision is harmless, it just results in a (failed) search.
///
#[derive(Default)]
pub struct InfoIndex {
    hashes: HashTable<u64>,
    hasher: DefaultHashBuilder,
    last: usize,
}

impl InfoIndex {
    pub fn new(infos: &VecDeque<DialogueInfo>) -> Self {
        let mut this = Self::default();
        this.reset(infos);
        this
    }

    /// Re-initialize for a different group, reusing the allocation.
    ///
    pub fn reset(&mut self, infos: &VecDeque<DialogueInfo>) {
        self.hashes.clear();
        for info in infos {
            self.insert(&info.id);
        }
        self.last = infos.len().saturating_sub(1);
    }

    /// Returns `true` if the id might exist in the group.
    ///
    #[inline]
    fn contains(&self, id: &str) -> bool {
        let hash = self.hasher.hash_one(id);
        self.hashes.find(hash, |&h| h == hash).is_some()
    }

    /// Adds the id, returning `true` if it was definitely not already present.
    ///
    #[inline]
    fn insert(&mut self, id: &str) -> bool {
        let hash = self.hasher.hash_one(id);
        match self.hashes.entry(hash, |&h| h == hash, |&h| h) {
            hash_table::Entry::Occupied(_) => false,
            hash_table::Entry::Vacant(entry) => {
                entry.insert(hash);
                true
            }
        }
    }

    /// Returns the position of the id, checking the `hint` position first.
    ///
    #[inline]
    fn find(infos: &VecDeque<DialogueInfo>, id: &str, hint: usize) -> Option<usize> {
        let is_match = |info: &DialogueInfo| info.id == id;
        if infos.get(hint).is_some_and(is_match) {
            Some(hint)
        } else {
            infos.iter().rposition(is_match)
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
