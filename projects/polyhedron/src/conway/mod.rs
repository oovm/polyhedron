use std::ops::{Add, AddAssign};
use shape_core::{Float, Point3D, Polygon};
mod seeds;

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
            ConwayNotation::Join => {}
            ConwayNotation::Ambo => {}
            ConwayNotation::Kis(_) => {}
            ConwayNotation::Needle => {}
            ConwayNotation::Zip => {}
            ConwayNotation::Truncate(_) => {}
            ConwayNotation::Ortho(_) => {}
            ConwayNotation::Extend(_) => {}
            ConwayNotation::Chamfer => {}
            ConwayNotation::Subdivide => {}
            ConwayNotation::Gyro(_) => {}
            ConwayNotation::Snub => {}
            ConwayNotation::Propeller => {}
            ConwayNotation::Meta => {}
            ConwayNotation::Bevel(_) => {}
        }
    }
}