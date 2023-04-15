use super::*;

impl ConwaySeed {
    pub fn as_polyhedron<T: Float>(&self) -> Polyhedron<T> {
        match self {
            ConwaySeed::Tetrahedron => {
                let vertices = vec![
                    Point3D::new(T::zero(), T::zero(), T::one()),
                    Point3D::new(T::zero(), T::one(), T::zero()),
                    Point3D::new(T::one(), T::zero(), T::zero()),
                    Point3D::new(T::zero(), T::zero(), -T::one()),
                ];
                let faces = vec![
                    vec![0, 1, 2],
                    vec![0, 2, 3],
                    vec![0, 3, 1],
                    vec![1, 3, 2],
                ];
                Polyhedron {
                    name: "Tetrahedron".to_string(),
                    vertices,
                    faces,
                }
            }
            ConwaySeed::Cube => {
                let vertices = vec![
                    Point3D::new(T::one(), T::one(), T::one()),
                    Point3D::new(T::one(), T::one(), -T::one()),
                    Point3D::new(T::one(), -T::one(), T::one()),
                    Point3D::new(T::one(), -T::one(), -T::one()),
                    Point3D::new(-T::one(), T::one(), T::one()),
                    Point3D::new(-T::one(), T::one(), -T::one()),
                    Point3D::new(-T::one(), -T::one(), T::one()),
                    Point3D::new(-T::one(), -T::one(), -T::one()),
                ];
                let faces = vec![
                    vec![0, 1, 3, 2],
                    vec![0, 2, 6, 4],
                    vec![0, 4, 5, 1],
                    vec![1, 5, 7, 3],
                    vec![2, 3, 7, 6],
                    vec![4, 6, 7, 5],
                ];
                Polyhedron {
                    name: "Cube".to_string(),
                    vertices,
                    faces,
                }
            }
            ConwaySeed::Octahedron => {
                let vertices = vec![
                    Point3D::new(T::zero(), T::one(), T::zero()),
                    Point3D::new(T::one(), T::zero(), T::zero()),
                    Point3D::new(T::zero(), T::zero(), T::one()),
                    Point3D::new(-T::one(), T::zero(), T::zero()),
                    Point3D::new(T::zero(), T::zero(), -T::one()),
                    Point3D::new(T::zero(), -T::one(), T::zero()),
                ];
                let faces = vec![
                    vec![0, 1, 2],
                    vec![0, 2, 3],
                    vec![0, 3, 4],
                    vec![0, 4, 1],
                    vec![5, 2, 1],
                    vec![5, 3, 2],
                    vec![5, 4, 3],
                    vec![5, 1, 4],
                ];
                Polyhedron {
                    name: "Octahedron".to_string(),
                    vertices,
                    faces,
                }
            }
            ConwaySeed::Dodecahedron => {
                let phi = (T::one() + T::sqrt(T::from(5).unwrap())) / T::from(2).unwrap();
                let vertices = vec![
                    Point3D::new(T::one(), T::one(), T::one()),
                    Point3D::new(T::one(), T::one(), -T::one()),
                    Point3D::new(T::one(), -T::one(), T::one()),
                    Point3D::new(T::one(), -T::one(), -T::one()),
                    Point3D::new(-T::one(), T::one(), T::one()),
                    Point3D::new(-T::one(), T::one(), -T::one()),
                    Point3D::new(-T::one(), -T::one(), T::one()),
                    Point3D::new(-T::one(), -T::one(), -T::one()),
                    Point3D::new(T::zero(), phi, T::one() / phi),
                    Point3D::new(T::zero(), phi, -T::one() / phi),
                    Point3D::new(T::zero(), -phi, T::one() / phi),
                    Point3D::new(T::zero(), -phi, -T::one() / phi),
                    Point3D::new(phi, T::one() / phi, T::zero()),
                    Point3D::new(phi, -T::one() / phi, T::zero()),
                    Point3D::new(-phi, T::one() / phi, T::zero()),
                    Point3D::new(-phi, -T::one() / phi, T::zero()),
                    Point3D::new(T::one() / phi, T::zero(), phi),
                    Point3D::new(-T::one() / phi, T::zero(), phi),
                    Point3D::new(T::one() / phi, T::zero(), -phi),
                    Point3D::new(-T::one() / phi, T::zero(), -phi),
                ];
                let faces = vec![
                    vec![0, 9, 16, 3, 12],
                    vec![0, 12, 2, 13, 6],
                    vec![0, 6, 18, 8, 9],
                    vec![1, 17, 10, 7, 15],
                    vec![1, 15, 4, 19, 11],
                    vec![1, 11, 14, 5, 17],
                    vec![2, 13, 1, 11, 14],
                    vec![2, 14, 5, 18, 6],
                    vec![3, 16, 4, 15, 7],
                    vec![3, 7, 10, 19, 12],
                    vec![4, 16, 9, 8, 19],
                    vec![5, 17, 1, 13, 2],
                    vec![5, 2, 6, 18, 5],
                    vec![7, 10, 17, 5, 18],
                    vec![7, 18, 8, 19, 10],
                    vec![8, 18, 5, 17, 11],
                    vec![8, 11, 14, 2, 12],
                    vec![8, 12, 3, 16, 9],
                    vec![9, 16, 4, 19, 8],
                    vec![10, 19, 11, 17, 7],
                    vec![13, 1, 15, 4, 16],
                ];
            }
            ConwaySeed::Icosahedron => {
                let phi = (T::one() + T::sqrt(T::from(5).unwrap())) / T::from(2).unwrap();
                let vertices = vec![
                    Point3D::new(T::zero(), T::one(), phi),
                    Point3D::new(T::zero(), T::one(), -phi),
                    Point3D::new(T::zero(), -T::one(), phi),
                    Point3D::new(T::zero(), -T::one(), -phi),
                    Point3D::new(T::one(), phi, T::zero()),
                    Point3D::new(T::one(), -phi, T::zero()),
                    Point3D::new(-T::one(), phi, T::zero()),
                    Point3D::new(-T::one(), -phi, T::zero()),
                    Point3D::new(phi, T::zero(), T::one()),
                    Point3D::new(-phi, T::zero(), T::one()),
                    Point3D::new(phi, T::zero(), -T::one()),
                    Point3D::new(-phi, T::zero(), -T::one()),
                ];
                let faces = vec![
                    vec![0, 8, 4],
                    vec![0, 5, 10],
                    vec![2, 4, 9],
                    vec![2, 11, 5],
                    vec![1, 6, 8],
                    vec![1, 10, 7],
                    vec![3, 9, 6],
                    vec![3, 7, 11],
                    vec![0, 10, 8],
                    vec![1, 8, 10],
                    vec![2, 9, 11],
                    vec![3, 11, 9],
                    vec![4, 2, 0],
                    vec![5, 0, 2],
                    vec![6, 1, 3],
                    vec![7, 3, 1],
                    vec![8, 6, 4],
                    vec![9, 4, 6],
                    vec![10, 5, 7],
                    vec![11, 7, 5],
                ];
                Polyhedron {
                    name: "Icosahedron".to_string(),
                    vertices,
                    faces,
                }
            }
        }
    }
}