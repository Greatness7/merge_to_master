use crate::prelude::*;

use glam::{Affine3A, EulerRot, Quat, Vec3};
use tes3::esp::*;
use uncased::AsUncased;

#[ext]
impl Cell {
    pub fn clean_duplicates(&mut self, max_abs_diff: f32) {
        let mut reference_groups = HashMap::new();
        for (key, reference) in &self.references {
            if reference.deleted.is_some() {
                continue;
            }
            reference_groups
                .entry(reference.id.as_uncased())
                .or_insert_with(Vec::new)
                .push((*key, reference.transform()));
        }
        for mut group in reference_groups.into_values().collect_vec() {
            while let Some(a) = group.pop() {
                let duplicates = group.extract_if(.., |b| a.1.abs_diff_eq(b.1, max_abs_diff));
                for (key, _) in duplicates {
                    if let Some(reference) = self.references.remove(&key) {
                        info!(
                            "Removed duplicate reference: '{}' at {:?} from cell '{}'",
                            reference.id,
                            reference.translation,
                            self.editor_id()
                        );
                    }
                }
            }
        }
    }
}

#[ext]
impl Reference {
    fn transform(&self) -> Affine3A {
        let [x, y, z] = self.rotation;
        Affine3A::from_scale_rotation_translation(
            Vec3::splat(self.scale.unwrap_or(1.0)),
            Quat::from_euler(EulerRot::XYZ, -x, -y, -z),
            Vec3::from(self.translation),
        )
    }
}
