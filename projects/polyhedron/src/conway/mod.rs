use shape_core::{Float, Point3D, Polygon};
use std::ops::{Add, AddAssign};

mod display;
mod iters;
mod seeds;

use num_traits::real::Real;
use std::{
    fmt::{Debug, Display, Formatter},
    mem::swap,
    slice::Iter,
};

#[derive(Copy, Clone, Debug)]
pub enum ConwaySeed {
    Tetrahedron,
    Cube,
    Octahedron,
    Dodecahedron,
    Icosahedron,
}

#[derive(Copy, Clone, Debug)]
pub enum ConwayNotation {
    // ============= [E = 1]
    /// Dual replaces each face with a vertex, and each vertex with a face.
    Dual,
    // ============= [E = 2]
    /// Join adds a new vertex to the center of each face.
    ///
    /// - E = 2
    Join,
    /// Snub replaces each vertex with a new vertex at the center of each face.
    /// - E = 2
    Ambo,
    // ============= [E = 3]
    Kis(usize),
    Needle,
    Zip,
    Truncate(usize),
    // ============= [E = 4]
    Ortho(usize),
    Extend(usize),
    Chamfer,
    Subdivide,
    Gyro(usize),
    Snub,
    Propeller,
    Meta,
    Bevel(usize),
}

pub struct ConwayPolyhedron {
    base: ConwaySeed,
    operations: Vec<ConwayNotation>,
}

impl ConwayPolyhedron {
    pub fn as_polyhedron<T>(&self) -> Polyhedron<T>
    where
        T: Real,
    {
        let mut polyhedron = self.base.as_polyhedron();
        for operation in self.operations.iter() {
            polyhedron += *operation;
        }
        polyhedron.name = self.to_string();
        polyhedron
    }
}

pub struct Polyhedron<T> {
    name: String,
    vertices: Vec<Point3D<T>>,
    face_index: Vec<Vec<usize>>,
}

impl<T: Real> Add<ConwayNotation> for Polyhedron<T> {
    type Output = Self;

    fn add(mut self, rhs: ConwayNotation) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: Real> AddAssign<ConwayNotation> for Polyhedron<T> {
    fn add_assign(&mut self, rhs: ConwayNotation) {
        match rhs {
            // All faces are rotated, so no need to change labels.
            ConwayNotation::Dual => {
                let mut new_vertices = Vec::new();
                let mut new_face_index = Vec::new();
                let mut index = 0;
                for face in self.faces() {
                    let face_center = face.center();
                    new_vertices.push(face_center);
                    let mut face_index = Vec::new();
                    for vertex in face.vertices() {
                        new_vertices.push(*vertex);
                        face_index.push(index);
                        index += 1;
                    }
                    new_face_index.push(face_index);
                }
                self.vertices = new_vertices;
                self.face_index = new_face_index;
            }
            ConwayNotation::Join => {
                todo!()
            }
            ConwayNotation::Ambo => {
                todo!()
            }
            ConwayNotation::Kis(k) => {
                todo!()
            }
            ConwayNotation::Needle => {
                todo!()
            }
            ConwayNotation::Zip => {
                todo!()
            }
            ConwayNotation::Truncate(_) => {
                todo!()
            }
            ConwayNotation::Ortho(_) => {
                todo!()
            }
            ConwayNotation::Extend(_) => {
                todo!()
            }
            ConwayNotation::Chamfer => {
                todo!()
            }
            ConwayNotation::Subdivide => {
                todo!()
            }
            ConwayNotation::Gyro(_) => {
                todo!()
            }
            ConwayNotation::Snub => {
                todo!()
            }
            ConwayNotation::Propeller => {
                todo!()
            }
            ConwayNotation::Meta => {
                todo!()
            }
            ConwayNotation::Bevel(_) => {
                todo!()
            }
        }
    }
}
