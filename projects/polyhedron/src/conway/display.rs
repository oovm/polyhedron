use super::*;

impl Display for ConwayPolyhedron {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "ConwayPolyhedron")
    }
}

impl ConwayNotation {
    pub fn as_chinese(&self) -> String {
        match self {
            // 产生对偶多面体-每个顶点创建一个新的面，或面的重心当作新的顶点。
            ConwayNotation::Dual => "对偶".to_string(),
            // 每个面都加入上当高的锥体，使相邻面的锥体各有一面互相共面，形成四边形。
            ConwayNotation::Join => "会合".to_string(),
            // 边是新的顶点，旧的顶点消失，或将边的中点当作新的顶点。
            ConwayNotation::Ambo => "截半".to_string(),
            // n角化, 每个面都加入角锥.
            ConwayNotation::Kis(n) => {
                format!("{}角化", n)
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
