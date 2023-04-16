use super::*;
use itertools::Itertools;
use num_traits::real::Real;
use shape_core::{Line, Line3D, Polygon3D};
use std::{collections::BTreeSet, iter::from_generator};

impl<T: Real> Polyhedron<T> {
    pub fn vertices(&self) -> impl Iterator<Item = Point3D<T>> + '_ {
        self.vertices.iter().map(|p| *p)
    }
    pub fn faces(&self) -> impl Iterator<Item = Polygon3D<T>> + '_ {
        from_generator(move || {
            for face in &self.face_index {
                yield Polygon3D { vertex: face.iter().map(|s| unsafe { *self.vertices.get_unchecked(*s) }).collect() }
            }
        })
    }
    pub fn edges(&self) -> impl Iterator<Item = Line3D<T>> + '_ {
        let mut unique = BTreeSet::new();
        from_generator(move || {
            for face in &self.face_index {
                for (s, e) in face.iter().circular_tuple_windows::<(_, _)>() {
                    // Ensure that the edge is in the correct order.
                    if s.le(e) {
                        match unique.get(&(*s, *e)) {
                            Some(_) => continue,
                            None => {
                                unique.insert((*s, *e));
                                yield unsafe { Line3D::new(*self.vertices.get_unchecked(*s), *self.vertices.get_unchecked(*e)) }
                            }
                        }
                    }
                }
            }
        })
    }
}
