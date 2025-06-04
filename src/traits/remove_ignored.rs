use tes3::esp::ObjectInfo;

use crate::prelude::*;

pub trait RemoveIgnored {
    /// Remove all objects that are marked as ignored.
    ///
    fn remove_ignored(&mut self);
}

impl<T> RemoveIgnored for Option<T>
where
    T: ObjectInfo,
{
    fn remove_ignored(&mut self) {
        _ = self.take_if(|x| x.ignored());
    }
}

impl RemoveIgnored for Exterior {
    fn remove_ignored(&mut self) {
        self.cell.remove_ignored();
        self.landscape.remove_ignored();
        self.pathgrid.remove_ignored();
    }
}

impl RemoveIgnored for Interior {
    fn remove_ignored(&mut self) {
        self.cell.remove_ignored();
        self.pathgrid.remove_ignored();
    }
}

impl RemoveIgnored for DialogueGroup {
    fn remove_ignored(&mut self) {
        self.infos.retain(|info| !info.ignored());
    }
}

impl RemoveIgnored for PluginData {
    fn remove_ignored(&mut self) {
        self.objects.retain(|_, object| {
            !object.ignored() //
        });
        self.cells.exteriors.retain(|_, exterior| {
            exterior.remove_ignored();
            exterior.count_objects() != 0
        });
        self.cells.interiors.retain(|_, interior| {
            interior.remove_ignored();
            interior.count_objects() != 0
        });
        self.dialogues.retain(|_, group| {
            group.remove_ignored();
            !group.dialogue.ignored() // Note: discards all infos if the dialogue itself is ignored.
        });
    }
}
