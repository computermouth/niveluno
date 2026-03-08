use modelz::{Indices, Model3D};
use raylib::prelude::*;
use glam::Vec3;

#[allow(dead_code)]
pub fn get_triangles(scene: Model3D) -> Vec<[Vector3; 3]> {
    let mut surfaces = vec![];

    for mesh in scene.meshes {
        let ind: Vec<usize> = match mesh.indices.unwrap() {
            Indices::U8(s) => s.iter().map(|i| *i as usize).collect(),
            Indices::U16(s) => s.iter().map(|i| *i as usize).collect(),
            Indices::U32(s) => s.iter().map(|i| *i as usize).collect(),
        };

        for tri in ind.chunks(3) {
            let v1 = match mesh.vertices[tri[0]].position {
                [x, y, z] => Vector3::from((x, y, z)),
            };
            let v2 = match mesh.vertices[tri[1]].position {
                [x, y, z] => Vector3::from((x, y, z)),
            };
            let v3 = match mesh.vertices[tri[2]].position {
                [x, y, z] => Vector3::from((x, y, z)),
            };

            surfaces.push([v1, v2, v3]);
        }
    }

    surfaces
}

pub trait ToVec3 {
    fn to_mcapv3(&self) -> Vec3;
}

#[allow(dead_code)]
pub trait ToVector3 {
    fn to_rayv3(&self) -> Vector3;
}

impl ToVec3 for Vector3 {
    fn to_mcapv3(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl ToVector3 for Vec3 {
    fn to_rayv3(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
}