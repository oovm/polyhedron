use std::collections::BTreeSet;
use std::iter::from_generator;
use itertools::Itertools;
use num_traits::real::Real;
use shape_core::{Line, Line3D};
use super::*;

impl<T: Real> Polyhedron<T> {
    pub fn vertices(&self) -> impl Iterator<Item=Point3D<T>> + '_ {
        self.vertices.iter().map(|p| *p)
    }
    // pub fn faces(&self) -> impl Iterator<Item=Polygon3D<T>> + '_ {
    //     from_generator(move || {
    //         for face in &self.face_index {
    //             yield face.iter().map(|s| unsafe { self.vertices.get_unchecked(*s).as_ref() })
    //         }
    //     })
    // }
    pub fn edges(&self) -> impl Iterator<Item=Line3D<T>> + '_ {
        let mut unique = BTreeSet::new();
        from_generator(move || {
            for face in &self.face_index {
                for (s, e) in face.iter().circular_tuple_windows::<(_, _)>() {
                    // Ensure that the edge is in the correct order.
                    if s.le(e) {
                        match unique.get(&(*s, *e)) {
                            Some(_) => {
                                continue;
                            }
                            None => {
                                unique.insert((*s, *e));
                                yield unsafe {
                                    Line3D {
                                        start: *self.vertices.get_unchecked(*s),
                                        end: *self.vertices.get_unchecked(*e),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}
