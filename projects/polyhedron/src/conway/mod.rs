use std::ops::{Add, AddAssign};
use shape_core::{Float, Point3D, Polygon};

mod seeds;

use std::fmt::{Display, Formatter};
use std::slice::Iter;

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
    faces: Vec<Vec<usize>>,
}

pub struct PolyhedronPoints<'i, T> {
    vertices: Iter<'i, Point3D<T>>
}
pub struct PolyhedronFaces<'i, T> {
    vertices: &'i [Point3D<T>],
    faces: Iter<'i, Vec<usize>>
}

pub struct PolyhedronEdges<'i, T> {
    vertices: &'i [Point3D<T>],
    faces: Iter<'i, Vec<usize>>
}

impl <T> Polyhedron<T> {
    pub fn vertices(&self) -> PolyhedronPoints<T> {
        PolyhedronPoints {
            vertices: self.vertices.iter()
        }
    }
    pub fn faces(&self) -> PolyhedronFaces<T> {
        PolyhedronFaces {
            vertices: &self.vertices,
            faces: self.faces.iter()
        }
    }
    pub fn edges(&self) -> PolyhedronEdges<T> {
        PolyhedronEdges {
            vertices: &self.vertices,
            faces: self.faces.iter()
        }
    }
}


impl<T: Float> AddAssign<ConwayNotation> for Polyhedron<T> {
    fn add_assign(&mut self, rhs: ConwayNotation) {
        match rhs {
            ConwayNotation::Seed(_) => {
                panic!("Cannot add a seed to a polyhedron.")
            }
            ConwayNotation::Dual => {
                let mut new_vertices = Vec::new();
                let mut new_faces = Vec::new();
                for face in &self.faces {
                    let mut new_face = Vec::new();
                    for vertex in face {
                        new_face.push(new_vertices.len());
                        new_vertices.push(self.vertices[*vertex]);
                    }
                    new_faces.push(new_face);
                }
                for vertex in &self.vertices {
                    let mut new_vertex = Vec::new();
                    for face in &self.faces {
                        if face.contains(&vertex) {
                            new_vertex.push(face.len());
                        }
                    }
                    new_faces.push(new_vertex);
                }
                self.vertices = new_vertices;
                self.faces = new_faces;
            }
            ConwayNotation::Join => {
                let mut new_vertices = Vec::new();
                let mut new_faces = Vec::new();
                for face in &self.faces {
                    let mut new_face = Vec::new();
                    for vertex in face {
                        new_face.push(new_vertices.len());
                        new_vertices.push(self.vertices[*vertex]);
                    }
                    new_face.push(new_vertices.len());
                    new_vertices.push(face.iter().map(|v| self.vertices[*v]).sum::<Point3D<T>>() / T::from_usize(face.len()).unwrap());
                    new_faces.push(new_face);
                }
                self.vertices = new_vertices;
                self.faces = new_faces;
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