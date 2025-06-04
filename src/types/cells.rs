use tes3::esp::{Cell, EditorId, Landscape, ObjectInfo, PathGrid, Reference};

use crate::prelude::*;

#[derive(Default)]
pub struct Cells {
    pub interiors: HashMap<UString, Interior>,
    pub exteriors: HashMap<(i32, i32), Exterior>,
}

#[derive(Default)]
pub struct Interior {
    pub cell: Option<Cell>,
    pub pathgrid: Option<PathGrid>,
}

#[derive(Default)]
pub struct Exterior {
    pub cell: Option<Cell>,
    pub landscape: Option<Landscape>,
    pub pathgrid: Option<PathGrid>,
}

impl Cells {
    pub fn get_interior(&self, name: &str) -> Option<&Interior> {
        self.interiors.get(name.as_uncased())
    }

    pub fn get_interior_mut(&mut self, name: &str) -> Option<&mut Interior> {
        self.interiors.get_mut(name.as_uncased())
    }

    pub fn get_exterior(&self, coords: (i32, i32)) -> Option<&Exterior> {
        self.exteriors.get(&coords)
    }

    pub fn get_exterior_mut(&mut self, coords: (i32, i32)) -> Option<&mut Exterior> {
        self.exteriors.get_mut(&coords)
    }

    pub fn get_or_create_exterior(&mut self, coords: (i32, i32)) -> &mut Exterior {
        self.exteriors.entry(coords).or_default()
    }

    pub fn get_or_create_interior(&mut self, name: &str) -> &mut Interior {
        self.interiors.get_or_insert_default(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Cell> {
        Iterator::chain(
            self.exteriors.values().filter_map(|exterior| exterior.cell.as_ref()),
            self.interiors.values().filter_map(|interior| interior.cell.as_ref()),
        )
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        Iterator::chain(
            self.exteriors
                .values_mut()
                .filter_map(|exterior| exterior.cell.as_mut()),
            self.interiors
                .values_mut()
                .filter_map(|interior| interior.cell.as_mut()),
        )
    }

    pub fn par_iter(&self) -> impl ParallelIterator<Item = &Cell> {
        ParallelIterator::chain(
            self.exteriors
                .par_values()
                .filter_map(|exterior| exterior.cell.as_ref()),
            self.interiors
                .par_values()
                .filter_map(|interior| interior.cell.as_ref()),
        )
    }

    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut Cell> {
        ParallelIterator::chain(
            self.exteriors
                .par_values_mut()
                .filter_map(|exterior| exterior.cell.as_mut()),
            self.interiors
                .par_values_mut()
                .filter_map(|interior| interior.cell.as_mut()),
        )
    }
}

impl Cells {
    /// Put 'moved references' into their the new cell's reference list.
    ///
    pub fn apply_moved_references(&mut self) {
        let moved_references = self
            .exteriors
            .values_mut()
            .filter_map(|exterior| exterior.cell.as_mut())
            .flat_map(|cell| {
                cell.references
                    .extract_if(|_, r| r.mast_index == 0 && r.moved_cell.is_some())
            })
            .collect_vec();

        for (key, mut reference) in moved_references {
            let coords = reference.moved_cell.unwrap();
            if let Some(exterior) = self.exteriors.get_mut(&coords)
                && let Some(cell) = exterior.cell.as_mut()
                && !cell.ignored()
            {
                info!(
                    "Applying moved reference '{}' ({}) for cell {:?}",
                    reference.id, key.1, coords
                );
                reference.moved_cell = None;
                cell.references.insert(key, reference);
            } else {
                panic!(
                    "Moved reference '{}' ({}) has invalid cell {:?}",
                    reference.id, key.1, coords
                );
            }
        }
    }

    /// Remove all duplicate references.
    ///
    /// (i.e. those with identical id and transform)
    ///
    pub fn remove_duplicate_references(&mut self) {
        const MAX_ABS_DIFF: f32 = 1e-5;

        let get_transform = |reference: &Reference| {
            use glam::{Affine3A, EulerRot, Quat, Vec3};
            let [x, y, z] = reference.rotation;
            Affine3A::from_scale_rotation_translation(
                Vec3::splat(reference.scale.unwrap_or(1.0)),
                Quat::from_euler(EulerRot::XYZ, -x, -y, -z),
                Vec3::from(reference.translation),
            )
        };

        let mut reference_groups = HashMap::new();

        for cell in self.iter_mut() {
            // Group references by their id.
            for (key, reference) in &cell.references {
                if !reference.deleted() {
                    reference_groups
                        .get_or_insert_with(&reference.id, Vec::new)
                        .push((*key, get_transform(reference)));
                }
            }
            // For each group, remove references that have identical transforms.
            for (_, mut group) in reference_groups.drain() {
                while let Some(a) = group.pop() {
                    for (key, _) in group.extract_if(.., |b| a.1.abs_diff_eq(b.1, MAX_ABS_DIFF)) {
                        if let Some(reference) = cell.references.remove(&key) {
                            info!(
                                "Removed duplicate reference: '{}' at {:?} from cell '{}'",
                                reference.id,
                                reference.translation,
                                cell.editor_id()
                            );
                        }
                    }
                }
            }
        }
    }
}
