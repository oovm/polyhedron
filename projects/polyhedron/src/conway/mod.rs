use std::ops::{Add, AddAssign};
use shape_core::{Float, Point3D, Polygon};

mod seeds;
mod iters;

use std::fmt::{Display, Formatter};
use std::slice::Iter;
use num_traits::real::Real;

pub enum ConwaySeed {
    Tetrahedron,
    Cube,
    Octahedron,
    Dodecahedron,
    Icosahedron,
}

pub enum ConwayNotation {
    // ============= [E = 1]
    /// Seed is the initial polyhedron.
    Seed(ConwaySeed),
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
            ConwayNotation::Seed(_) => {
                panic!("Cannot add a seed to a polyhedron.")
            }
            ConwayNotation::Dual => {
                todo!()
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