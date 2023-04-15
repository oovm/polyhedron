use std::collections::BTreeSet;
use itertools::Itertools;
use shape_core::{Line, Line3D};
use super::*;

pub struct PolyhedronPoints<'i, T> {
    vertices: Iter<'i, Point3D<T>>,
}

impl<'i, T> Iterator for PolyhedronPoints<'i, T> {
    type Item = &'i Point3D<T>;
    fn next(&mut self) -> Option<Self::Item> {
        self.vertices.next()
    }
}


pub struct PolyhedronFaces<'i, T> {
    vertices: &'i [Point3D<T>],
    faces: Iter<'i, Vec<usize>>,
}

// impl<'i, T> Iterator for PolyhedronFaces<'i, T> {
//     type Item = Polygon3D<&'i T>;
//     fn next(&mut self) -> Option<Self::Item> {
//         let face = self.faces.next()?;
//         Polygon3D::new(face.iter().map(|v| unsafe {
//             self.vertices.get_unchecked(*v)
//         }))
//     }
// }

pub struct PolyhedronEdges<'i, T> {
    vertices: &'i [Point3D<T>],
    faces: Iter<'i, Vec<usize>>,
    exists: BTreeSet<(usize, usize)>
}

impl<'i, T> Iterator for PolyhedronEdges<'i, T> {
    type Item = Line3D<&'i T>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<T> Polyhedron<T> {
    pub fn vertices(&self) -> PolyhedronPoints<T> {
        PolyhedronPoints {
            vertices: self.vertices.iter()
        }
    }
    pub fn faces(&self) -> PolyhedronFaces<T> {
        PolyhedronFaces {
            vertices: &self.vertices,
            faces: self.face_index.iter(),
        }
    }
    pub fn edges(&self) -> PolyhedronEdges<T> {
        let unique = BTreeSet::new();

        let edges = self
            .face_index
            .iter()
            .map(|face| {
                face.iter()
                    // Grab two index entries.
                    .circular_tuple_windows::<(_, _)>()
                    .filter(|t| t.0 < t.1)
                    // Create an edge from them.
                    .map(|t| [*t.0, *t.1])
                    .collect::<Vec<_>>()
            })
            .flatten()
            .unique();
        todo!()
    }
}
