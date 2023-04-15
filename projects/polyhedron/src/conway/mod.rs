
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

pub struct ConwayPolyhedron {
    pub name: &'static str,
    pub notation: ConwayNotation,
    pub vertices: Vec<Point3D>,
    pub faces: Vec<Vec<usize>>,
}